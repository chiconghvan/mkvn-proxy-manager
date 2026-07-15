use std::{collections::HashSet, sync::Arc};

use futures::future::join_all;
use tauri::AppHandle;
use tokio::sync::Semaphore;
use tracing::{error, info, warn};

use crate::{
    error::{AppError, AppResult},
    events::{self, DATABASE_UPDATED, SYNC_COMPLETED, SYNC_PROGRESS, SYNC_STARTED},
    matcher::{MatchResult, ProxyProfileMatcher},
    models::{ManagerSyncResult, MkvnOrder, SyncProgress, SyncResult},
    state::AppState,
};

pub struct SyncEngine;

impl SyncEngine {
    pub async fn sync_all(state: &AppState, app: &AppHandle) -> AppResult<SyncResult> {
        events::emit(app, SYNC_STARTED, SyncProgress { message: "Starting synchronization".into(), current: 0, total: 100 });
        let mut result = SyncResult::default();

        Self::sync_managers(state, app, &mut result).await;
        Self::sync_orders(state, app, &mut result).await?;
        Self::match_cached(state, app, &mut result)?;

        events::emit(app, DATABASE_UPDATED, ());
        events::emit(app, SYNC_COMPLETED, result.clone());
        Ok(result)
    }

    async fn sync_managers(state: &AppState, app: &AppHandle, result: &mut SyncResult) {
        let managers = state.managers.read().all();
        let total = managers.len().max(1);
        let tasks = managers.into_iter().enumerate().map(|(idx, manager)| {
            let db = Arc::clone(&state.db);
            let app = app.clone();
            async move {
                events::emit(&app, SYNC_PROGRESS, SyncProgress {
                    message: format!("Syncing {}", manager.display_name()),
                    current: idx,
                    total,
                });
                let manager_name = manager.display_name().to_string();
                let groups = manager.load_groups().await;
                let profiles = manager.load_profiles().await;
                match (groups, profiles) {
                    (Ok(groups), Ok(profiles)) => {
                        if let Err(err) = db.upsert_groups(&groups).and_then(|_| db.upsert_profiles(&profiles)) {
                            let msg = err.to_string();
                            error!(manager = %manager_name, error = %msg, "manager cache write failed");
                            ManagerSyncResult { manager: manager_name, groups: groups.len(), profiles: profiles.len(), error: Some(msg) }
                        } else {
                            ManagerSyncResult { manager: manager_name, groups: groups.len(), profiles: profiles.len(), error: None }
                        }
                    }
                    (Err(err), _) | (_, Err(err)) => {
                        let msg = err.to_string();
                        warn!(manager = %manager_name, error = %msg, "manager sync failed");
                        ManagerSyncResult { manager: manager_name, groups: 0, profiles: 0, error: Some(msg) }
                    }
                }
            }
        });
        result.managers = join_all(tasks).await;
        result.errors.extend(result.managers.iter().filter_map(|m| m.error.clone()));
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

        // Skip orders already cached — chỉ fetch proxy cho orders chưa có trong DB
        let cached_orders = state.db.get_order_codes_with_proxies()?;
        let to_fetch: Vec<MkvnOrder> = orders.into_iter().filter(|o| !cached_orders.contains(&o.code)).collect();
        if to_fetch.is_empty() {
            // đếm từ DB thay vì API
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

    fn match_cached(state: &AppState, app: &AppHandle, result: &mut SyncResult) -> AppResult<()> {
        events::emit(app, SYNC_PROGRESS, SyncProgress { message: "Matching proxies to profiles".into(), current: 85, total: 100 });
        let profiles = state.db.get_profiles()?;
        let groups = state.db.get_groups()?;
        let proxies = state.db.get_stored_proxies()?;
        let matcher = ProxyProfileMatcher::build(&profiles, &groups);
        let mut matches = Vec::new();
        for proxy in proxies {
            let (Some(host), Some(port)) = (proxy.host, proxy.port) else { continue; };
            let profile = matcher.match_proxy(&host, port);
            if profile.is_some() {
                result.matched += 1;
            }
            matches.push(MatchResult {
                proxy_host: host,
                proxy_port: port,
                order_code: proxy.order_code,
                manager: profile.map(|p| p.manager.clone()),
                profile_id: profile.map(|p| p.profile_id.clone()),
                profile_name: profile.map(|p| p.profile_name.clone()),
                group_name: profile.and_then(|p| p.group_name.clone()),
            });
        }
        state.db.save_match_results(&matches)?;
        info!(matched = result.matched, "matching completed");
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
