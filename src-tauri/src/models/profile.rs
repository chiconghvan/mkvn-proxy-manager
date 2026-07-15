use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedProfile {
    pub id: String,
    pub name: String,
    pub raw_proxy: Option<String>,
    pub host: Option<String>,
    pub port: Option<u16>,
    pub group_id: Option<String>,
    pub manager: String,
}
