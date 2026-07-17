use std::{collections::HashSet, sync::Arc, time::Duration};

use futures::future::join_all;
use tauri::AppHandle;
use tokio::sync::Semaphore;
use tracing::{debug, info, warn};

use crate::{
    error::{AppError, AppResult},
    events::{self, DATABASE_UPDATED, SYNC_COMPLETED, SYNC_PROGRESS, SYNC_STARTED},
    managers::ProfileManager,
    matcher::{extract_host_port, MatchResult, ProxyProfileMatcher},
    models::{ManagerSyncResult, MkvnOrder, SyncProgress, SyncResult, UnifiedGroup, UnifiedProfile},
    state::AppState,
};

pub struct SyncEngine;

impl SyncEngine {
    pub async fn sync_all(state: &AppState, app: &AppHandle) -> AppResult<SyncResult> {
        events::emit(app, SYNC_STARTED, SyncProgress { message: "Starting synchronization".into(), current: 0, total: 100 });
        let mut result = SyncResult::default();

        let (profiles, groups) = Self::sync_managers(state, app, &mut result).await;
        Self::sync_orders(state, app, &mut result).await?;
        Self::match_cached(state, app, &mut result, &profiles, &groups)?;

        events::emit(app, DATABASE_UPDATED, ());
        events::emit(app, SYNC_COMPLETED, result.clone());
        Ok(result)
    }

    async fn sync_managers(state: &AppState, app: &AppHandle, result: &mut SyncResult) -> (Vec<UnifiedProfile>, Vec<UnifiedGroup>) {
        let managers = state.managers.read().all();
        let total = managers.len().max(1);
        let tasks = managers.into_iter().enumerate().map(|(idx, manager)| {
            let app = app.clone();
            async move {
                events::emit(&app, SYNC_PROGRESS, SyncProgress {
                    message: format!("Syncing {}", manager.display_name()),
                    current: idx,
                    total,
                });
                let manager_name = manager.display_name().to_string();

                let groups = Self::load_groups_with_retry(&*manager, &manager_name).await;
                let profiles = Self::load_profiles_with_retry(&*manager, &manager_name).await;

                let groups_ok = groups.as_ref().ok().map(|g| g.len()).unwrap_or(0);
                let profiles_ok = profiles.as_ref().ok().map(|p| p.len()).unwrap_or(0);
                let mut errors: Vec<String> = Vec::new();

                if let Err(err) = &groups {
                    warn!(manager = %manager_name, error = %err, "groups sync failed");
                    errors.push(err.to_string());
                }
                if let Err(err) = &profiles {
                    warn!(manager = %manager_name, error = %err, "profiles sync failed");
                    errors.push(err.to_string());
                }

                let error = if errors.is_empty() { None } else { Some(errors.join("; ")) };
                let m_result = ManagerSyncResult { manager: manager_name.clone(), groups: groups_ok, profiles: profiles_ok, error };

                let groups = groups.unwrap_or_default();
                let profiles = profiles.unwrap_or_default();

                let with_host_port = profiles.iter().filter(|p| p.host.is_some() && p.port.is_some()).count();
                info!(manager = %manager_name, total = profiles.len(), with_host_port, "manager profiles loaded");

                (m_result, profiles, groups)
            }
        });
        let results: Vec<_> = join_all(tasks).await;
        let mut all_profiles = Vec::new();
        let mut all_groups = Vec::new();
        for (m_result, profiles, groups) in results {
            all_profiles.extend(profiles);
            all_groups.extend(groups);
            result.managers.push(m_result);
        }
        result.errors.extend(result.managers.iter().filter_map(|m| m.error.clone()));
        (all_profiles, all_groups)
    }

    async fn load_groups_with_retry(manager: &dyn ProfileManager, name: &str) -> AppResult<Vec<UnifiedGroup>> {
        let mut last_err = None;
        for attempt in 0..3 {
            match manager.load_groups().await {
                Ok(groups) => return Ok(groups),
                Err(err) => {
                    warn!(manager = %name, attempt, error = %err, "load groups retry");
                    last_err = Some(err);
                    if attempt < 2 {
                        tokio::time::sleep(Duration::from_millis(500 + attempt as u64 * 500)).await;
                    }
                }
            }
        }
        Err(last_err.unwrap())
    }

    async fn load_profiles_with_retry(manager: &dyn ProfileManager, name: &str) -> AppResult<Vec<UnifiedProfile>> {
        let mut last_err = None;
        for attempt in 0..3 {
            match manager.load_profiles().await {
                Ok(profiles) => return Ok(profiles),
                Err(err) => {
                    warn!(manager = %name, attempt, error = %err, "load profiles retry");
                    last_err = Some(err);
                    if attempt < 2 {
                        tokio::time::sleep(Duration::from_millis(500 + attempt as u64 * 500)).await;
                    }
                }
            }
        }
        Err(last_err.unwrap())
    }

    async fn sync_orders(state: &AppState, app: &AppHandle, result: &mut SyncResult) -> AppResult<()> {
        let Some(client) = state.mkvn_client() else {
            result.errors.push("MKVN token is empty; proxy provider sync skipped".to_string());
            return Ok(());
        };

        events::emit(app, SYNC_PROGRESS, SyncProgress { message: "Loading MKVN orders".into(), current: 25, total: 100 });
        let mut orders = client.get_orders().await?;
        orders.retain(is_active_order);
        let active_codes: HashSet<String> = orders.iter().map(|o| o.code.clone()).collect();
        result.orders = orders.len();
        state.db.delete_expired_orders(&active_codes)?;
        state.db.upsert_orders(&orders)?;

        let cached_orders = state.db.get_order_codes_with_proxies()?;
        let to_fetch: Vec<MkvnOrder> = orders.into_iter().filter(|o| !cached_orders.contains(&o.code)).collect();
        if to_fetch.is_empty() {
            result.proxies = state.db.get_stored_proxies()?.len();
            return Ok(());
        }

        events::emit(app, SYNC_PROGRESS, SyncProgress { message: "Loading MKVN proxy details".into(), current: 45, total: 100 });
        let fetch_total = to_fetch.len();
        let semaphore = Arc::new(Semaphore::new(8));
        let tasks = to_fetch.into_iter().enumerate().map(|(idx, order)| {
            let client = Arc::clone(&client);
            let db = Arc::clone(&state.db);
            let semaphore = Arc::clone(&semaphore);
            let app = app.clone();
            async move {
                let _permit = semaphore.acquire_owned().await.expect("semaphore closed");
                events::emit(&app, SYNC_PROGRESS, SyncProgress {
                    message: format!("Loading proxies for {}", order.code),
                    current: idx,
                    total: fetch_total,
                });
                let mut last_err = None;
                for _ in 0..5 {
                    match client.get_proxies(&order.code).await {
                        Ok(proxies) => {
                            let count = proxies.len();
                            return db.upsert_proxies(&order.code, &proxies).map(|_| count).map_err(AppError::from);
                        }
                        Err(err) => last_err = Some(err),
                    }
                }
                Err(last_err.unwrap_or_else(|| AppError::Api(format!("failed to load proxies for {}", order.code))))
            }
        });
        let counts = join_all(tasks).await;
        for count in counts {
            match count {
                Ok(count) => result.proxies += count,
                Err(err) => {
                    warn!(error = %err, "proxy detail sync failed");
                    result.errors.push(err.to_string());
                }
            }
        }
        Ok(())
    }

    fn match_cached(state: &AppState, app: &AppHandle, result: &mut SyncResult, profiles: &[UnifiedProfile], groups: &[UnifiedGroup]) -> AppResult<()> {
        events::emit(app, SYNC_PROGRESS, SyncProgress { message: "Matching proxies to profiles".into(), current: 85, total: 100 });

        let mut by_manager: std::collections::HashMap<&str, usize> = std::collections::HashMap::new();
        for p in profiles {
            let has_host = p.host.is_some() && p.port.is_some();
            *by_manager.entry(&p.manager).or_default() += if has_host { 1 } else { 0 };
        }
        for (mgr, cnt) in &by_manager {
            info!(manager = %mgr, with_host_port = cnt, total = profiles.iter().filter(|p| p.manager == *mgr).count(), "matcher input");
        }

        let proxies = state.db.get_stored_proxies()?;
        info!(proxy_count = proxies.len(), "proxies loaded from DB");

        let matcher = ProxyProfileMatcher::build(profiles, groups);
        info!(matcher_entries = matcher.len(), "matcher built");

        let mut matches = Vec::new();
        for proxy in proxies {
            let (Some(host), Some(port)) = (proxy.host, proxy.port) else { continue; };

            // Try matching by stored host:port (domain-based from raw_proxy)
            let mut profiles = matcher.match_proxy(&host, port);
            let mut matched_via_ip = false;

            // If no match by domain, try IP-based host:port from raw_proxy_ip
            if profiles.is_none() {
                if let Some(ip) = &proxy.raw_proxy_ip {
                    if let Some((ip_host, ip_port)) = extract_host_port(ip) {
                        profiles = matcher.match_proxy(&ip_host, ip_port);
                        matched_via_ip = profiles.is_some();
                    }
                }
            }

            if let Some(profile_list) = &profiles {
                result.matched += 1;
                for p in profile_list.iter() {
                    if matched_via_ip {
                        info!(domain_host = %host, domain_port = port, manager = %p.manager, profile = %p.profile_name, "MATCHED via IP");
                    } else {
                        info!(host, port, manager = %p.manager, profile = %p.profile_name, "MATCHED");
                    }
                    // Always store domain-based host:port in match_results for JOIN compatibility
                    matches.push(MatchResult {
                        proxy_host: host.clone(),
                        proxy_port: port,
                        order_code: proxy.order_code.clone(),
                        manager: Some(p.manager.clone()),
                        profile_id: Some(p.profile_id.clone()),
                        profile_name: Some(p.profile_name.clone()),
                        group_name: p.group_name.clone(),
                    });
                }
            } else {
                debug!(host, port, "UNMATCHED");
                matches.push(MatchResult {
                    proxy_host: host,
                    proxy_port: port,
                    order_code: proxy.order_code,
                    manager: None,
                    profile_id: None,
                    profile_name: None,
                    group_name: None,
                });
            }
        }
        state.db.save_match_results(&matches)?;
        info!(matched = result.matched, unmatched = matches.len() - result.matched, "matching completed");
        Ok(())
    }
}

fn is_active_order(order: &MkvnOrder) -> bool {
    let renewal = order.renewal.as_deref().unwrap_or_default();
    if renewal.eq_ignore_ascii_case("EXPIRE") {
        return false;
    }
    order.time_con_lai.unwrap_or(0) > 0
}