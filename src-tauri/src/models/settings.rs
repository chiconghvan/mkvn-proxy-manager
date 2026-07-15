use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    #[serde(default)]
    pub mkvn_token: String,
    #[serde(default = "return_true")]
    pub gpm_standard_enabled: bool,
    #[serde(default)]
    pub gpm_standard_url: String,
    #[serde(default = "return_true")]
    pub gpm_global_enabled: bool,
    #[serde(default)]
    pub gpm_global_url: String,
    #[serde(default = "return_true")]
    pub donut_enabled: bool,
    #[serde(default)]
    pub donut_url: String,
    #[serde(default)]
    pub sqlite_path: Option<String>,
    #[serde(default = "return_sync_interval")]
    pub auto_sync_interval_secs: u64,
    #[serde(default)]
    pub theme: String,
    #[serde(default)]
    pub column_widths: HashMap<String, f64>,
}

fn return_true() -> bool { true }
fn return_sync_interval() -> u64 { 300 }

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            mkvn_token: String::new(),
            gpm_standard_enabled: true,
            gpm_standard_url: "http://127.0.0.1:19995/api/v3".to_string(),
            gpm_global_enabled: true,
            gpm_global_url: "http://127.0.0.1:9495/api/v1".to_string(),
            donut_enabled: true,
            donut_url: "http://127.0.0.1:10108/v1".to_string(),
            sqlite_path: None,
            auto_sync_interval_secs: 300,
            theme: "light".to_string(),
            column_widths: HashMap::new(),
        }
    }
}
