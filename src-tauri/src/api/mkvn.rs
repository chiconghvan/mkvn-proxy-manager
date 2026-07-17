use std::{sync::Arc, time::Duration};

use reqwest::Client;
use serde::Deserialize;
use tokio::time::sleep;

use crate::{
    api::rate_limiter::RateLimiter,
    error::{AppError, AppResult},
    matcher::extract_host_port,
    models::{Balance, MkvnOrder, Product, ProxyDetail},
};

#[derive(Clone)]
pub struct MkvnClient {
    http: Client,
    base_url: String,
    token: String,
    limiter: Arc<RateLimiter>,
}

#[derive(Debug, Deserialize)]
struct BalanceResponse {
    username: Option<String>,
    level: Option<String>,
    balance: Option<f64>,
    chietkhau: Option<f64>,
}

#[derive(Debug, Deserialize)]
struct ProductsResponse {
    data: Vec<Product>,
}

#[derive(Debug, Deserialize)]
struct OrdersResponse {
    data: Vec<MkvnOrder>,
}

#[derive(Debug, Deserialize)]
struct ProxiesResponse {
    order_code: Option<String>,
    proxies: Option<Vec<String>>,
    proxiesip: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct BuyResponse {
    order_code: Option<String>,
    code: Option<String>,
    #[allow(dead_code)]
    proxies: Option<Vec<String>>,
    #[allow(dead_code)]
    proxiesip: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct RenewPlusResponse {
    status: String,
    #[serde(rename = "statusCode")]
    #[allow(dead_code)]
    status_code: Option<i64>,
    message: Option<String>,
    #[allow(dead_code)]
    order_code: Option<String>,
    #[allow(dead_code)]
    remaining_balance: Option<f64>,
}

impl MkvnClient {
    pub fn new(http: Client, base_url: String, token: String, limiter: Arc<RateLimiter>) -> Self {
        Self { http, base_url, token, limiter }
    }

    pub fn is_configured(&self) -> bool {
        !self.token.trim().is_empty()
    }

    async fn request_get<T: for<'de> Deserialize<'de>>(&self, path: &str, params: &[(&str, String)]) -> AppResult<T> {
        if !self.is_configured() {
            return Err(AppError::NotConfigured("MKVN token is empty".to_string()));
        }
        self.limiter.acquire().await;
        let url = format!("{}{}", self.base_url, path);
        let mut query = vec![("token", self.token.clone())];
        query.extend(params.iter().cloned());
        let resp = self.http.get(url).query(&query).send().await?.error_for_status()?;
        Ok(resp.json::<T>().await?)
    }

    async fn request_post<T: for<'de> Deserialize<'de>>(&self, path: &str, params: &[(&str, String)]) -> AppResult<T> {
        if !self.is_configured() {
            return Err(AppError::NotConfigured("MKVN token is empty".to_string()));
        }
        self.limiter.acquire().await;
        let url = format!("{}{}", self.base_url, path);
        let mut query = vec![("token", self.token.clone())];
        query.extend(params.iter().cloned());
        let resp = self.http.post(url).query(&query).send().await?.error_for_status()?;
        Ok(resp.json::<T>().await?)
    }

    pub async fn get_balance(&self) -> AppResult<Balance> {
        let resp: BalanceResponse = self.request_get("/getbalance", &[]).await?;
        Ok(Balance {
            username: resp.username,
            level: resp.level,
            balance: resp.balance.unwrap_or_default(),
            chietkhau: resp.chietkhau,
        })
    }

    pub async fn get_products(&self) -> AppResult<Vec<Product>> {
        let resp: ProductsResponse = self.request_get("/products", &[]).await?;
        Ok(resp.data)
    }

    pub async fn get_orders(&self) -> AppResult<Vec<MkvnOrder>> {
        let resp: OrdersResponse = self.request_get("/getlistorders", &[]).await?;
        Ok(resp.data)
    }

    pub async fn get_proxies(&self, order_code: &str) -> AppResult<Vec<ProxyDetail>> {
        let resp: ProxiesResponse = self.request_get("/proxies", &[("ordercode", order_code.to_string())]).await?;
        Ok(proxy_details_from_response(
            resp.order_code.as_deref().unwrap_or(order_code),
            resp.proxies.unwrap_or_default(),
            resp.proxiesip.unwrap_or_default(),
        ))
    }

    pub async fn buy(&self, product_id: i64, renewal: bool, note: &str) -> AppResult<String> {
        let mut last_err = None;
        for attempt in 0..3 {
            let result: AppResult<BuyResponse> = self
                .request_post(
                    "/buy",
                    &[
                        ("id_product", product_id.to_string()),
                        ("quantity", "1".to_string()),
                        ("renewal", if renewal { "on" } else { "off" }.to_string()),
                        ("note", note.to_string()),
                    ],
                )
                .await;
            match result {
                Ok(resp) => {
                    if let Some(code) = resp.order_code.or(resp.code) {
                        return Ok(code);
                    }
                    last_err = Some(AppError::Api("buy response missing order code".to_string()));
                }
                Err(err) => last_err = Some(err),
            }
            if attempt < 2 {
                sleep(Duration::from_millis(500 + attempt * 250)).await;
            }
        }
        Err(last_err.unwrap_or_else(|| AppError::Api("buy failed".to_string())))
    }

    pub async fn renew_plus(&self, order_code: &str, months: i64) -> AppResult<()> {
        let resp: RenewPlusResponse = self
            .request_post(
                "/renewalplus",
                &[("ordercode", order_code.to_string()), ("month", months.to_string())],
            )
            .await?;
        if resp.status != "SUCCESS" {
            let msg = resp.message.unwrap_or_else(|| "unknown error".to_string());
            return Err(AppError::Api(format!("Renew failed for {}: {}", order_code, msg)));
        }
        Ok(())
    }

    pub async fn renewal_on_off(&self, order_code: &str, enable: bool) -> AppResult<()> {
        let _: serde_json::Value = self
            .request_post(
                "/renewalonoff",
                &[
                    ("ordercode", order_code.to_string()),
                    ("renewal", if enable { "on" } else { "off" }.to_string()),
                ],
            )
            .await?;
        Ok(())
    }
}

pub fn proxy_details_from_response(order_code: &str, proxies: Vec<String>, proxiesip: Vec<String>) -> Vec<ProxyDetail> {
    proxies
        .into_iter()
        .enumerate()
        .map(|(idx, raw)| {
            let raw_ip = proxiesip.get(idx).cloned();
            let parsed = extract_host_port(&raw);
            ProxyDetail {
                order_code: order_code.to_string(),
                raw_proxy: raw,
                raw_proxy_ip: raw_ip,
                host: parsed.as_ref().map(|(host, _)| host.clone()),
                port: parsed.map(|(_, port)| port),
            }
        })
        .collect()
}
