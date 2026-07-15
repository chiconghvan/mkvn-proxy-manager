use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppUpdateInfo {
    pub current_version: String,
    pub new_version: String,
    pub release_notes: String,
    pub download_url: String,
    pub published_at: String,
    pub release_page_url: Option<String>,
    pub update_available: bool,
}
