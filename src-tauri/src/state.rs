use std::{path::PathBuf, sync::Arc, time::Duration};

use parking_lot::RwLock;
use reqwest::Client;

use crate::{
    api::{donut::DonutClient, gpm_global::GpmGlobalClient, gpm_standard::GpmStandardClient, mkvn::MkvnClient, rate_limiter::RateLimiter},
    database::Database,
    error::AppResult,
    managers::{DonutManager, GpmGlobalManager, GpmStandardManager, ManagerRegistry},
    models::AppSettings,
};

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Database>,
    pub http: Client,
    pub rate_limiter: Arc<RateLimiter>,
    pub settings: Arc<RwLock<AppSettings>>,
    pub mkvn: Arc<RwLock<Option<Arc<MkvnClient>>>>,
    pub managers: Arc<RwLock<ManagerRegistry>>,
}

impl AppState {
    pub fn new(db_path: PathBuf) -> AppResult<Self> {
        let db = Arc::new(Database::open(db_path)?);
        let http = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()?;
        let rate_limiter = RateLimiter::new(15, Duration::from_secs(10));
        let settings = AppSettings::default();
        let state = Self {
            db,
            http,
            rate_limiter,
            settings: Arc::new(RwLock::new(settings.clone())),
            mkvn: Arc::new(RwLock::new(None)),
            managers: Arc::new(RwLock::new(ManagerRegistry::new())),
        };
        state.rebuild_clients(&settings);
        Ok(state)
    }

    pub fn rebuild_clients(&self, settings: &AppSettings) {
        let mkvn = if settings.mkvn_token.trim().is_empty() {
            None
        } else {
            Some(Arc::new(MkvnClient::new(
                self.http.clone(),
                "https://proxy.mkvn.net/api/apiv1".to_string(),
                settings.mkvn_token.clone(),
                Arc::clone(&self.rate_limiter),
            )))
        };
        *self.mkvn.write() = mkvn;

        let mut registry = ManagerRegistry::new();
        if settings.gpm_standard_enabled {
            registry.register(Arc::new(GpmStandardManager::new(GpmStandardClient::new(
                self.http.clone(),
                settings.gpm_standard_url.clone(),
            ))));
        }
        if settings.gpm_global_enabled {
            registry.register(Arc::new(GpmGlobalManager::new(GpmGlobalClient::new(
                self.http.clone(),
                settings.gpm_global_url.clone(),
            ))));
        }
        if settings.donut_enabled {
            registry.register(Arc::new(DonutManager::new(DonutClient::new(
                self.http.clone(),
                settings.donut_url.clone(),
            ))));
        }
        *self.managers.write() = registry;
        *self.settings.write() = settings.clone();
    }

    pub fn mkvn_client(&self) -> Option<Arc<MkvnClient>> {
        self.mkvn.read().clone()
    }
}
