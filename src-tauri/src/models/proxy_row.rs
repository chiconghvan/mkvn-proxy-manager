use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProxyRow {
    pub order_code: String,
    pub raw_proxy: String,
    pub raw_proxy_ip: Option<String>,
    pub proxy_type: Option<String>,
    pub profile_id: Option<String>,
    pub profile_name: Option<String>,
    pub group_name: Option<String>,
    pub manager: Option<String>,
    pub purchase_date: Option<String>,
    pub remaining_days: i64,
    pub renewal: Option<String>,
    pub status: String,
    pub note: Option<String>,
    pub price: Option<f64>,
}
