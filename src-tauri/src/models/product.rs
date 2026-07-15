use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Product {
    pub id_product: i64,
    pub name_product: String,
    pub price: Option<f64>,
    #[serde(rename = "type")]
    pub proxy_type: Option<String>,
    pub countrycode: Option<String>,
    pub description: Option<String>,
    pub buy_max: Option<i64>,
    pub buy_min: Option<i64>,
    pub catalogue: Option<String>,
    pub store_quantity: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Balance {
    pub username: Option<String>,
    pub level: Option<String>,
    pub balance: f64,
    pub chietkhau: Option<f64>,
}
