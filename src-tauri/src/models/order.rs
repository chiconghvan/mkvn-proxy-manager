use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MkvnOrder {
    pub username: Option<String>,
    pub code: String,
    pub name_product: Option<String>,
    pub quantity: Option<i64>,
    pub price: Option<f64>,
    pub time_buy: Option<String>,
    pub time_dau_ky: Option<String>,
    pub time_cuoi_ky: Option<String>,
    #[serde(default, deserialize_with = "deserialize_remaining_days")]
    pub time_con_lai: Option<i64>,
    pub renewal: Option<String>,
    pub note: Option<String>,
    #[serde(rename = "type")]
    pub proxy_type: Option<String>,
}

fn deserialize_remaining_days<'de, D>(deserializer: D) -> Result<Option<i64>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde_json::Value;
    let v: Option<Value> = Option::deserialize(deserializer)?;
    match v {
        None => Ok(None),
        Some(Value::Number(n)) => n.as_i64().map(Some).ok_or_else(|| serde::de::Error::custom("expected integer")),
        Some(Value::String(s)) => {
            let digits: String = s.chars().take_while(|c| c.is_ascii_digit()).collect();
            Ok(Some(digits.parse().unwrap_or(0)))
        }
        Some(_) => Ok(Some(0)),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyDetail {
    pub order_code: String,
    pub raw_proxy: String,
    pub raw_proxy_ip: Option<String>,
    pub host: Option<String>,
    pub port: Option<u16>,
}
