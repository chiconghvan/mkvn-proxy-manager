use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SyncProgress {
    pub message: String,
    pub current: usize,
    pub total: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManagerSyncResult {
    pub manager: String,
    pub profiles: usize,
    pub groups: usize,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SyncResult {
    pub managers: Vec<ManagerSyncResult>,
    pub orders: usize,
    pub proxies: usize,
    pub matched: usize,
    pub errors: Vec<String>,
}
