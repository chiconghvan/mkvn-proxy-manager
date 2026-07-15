use reqwest::Client;
use serde::Deserialize;
use serde_json::Value;

use crate::error::AppResult;

#[derive(Clone)]
pub struct GpmGlobalClient {
    http: Client,
    base_url: String,
}

#[derive(Debug, Deserialize)]
pub struct GpmGlobalProfile {
    pub id: Value,
    pub name: Option<String>,
    pub raw_proxy: Option<String>,
    pub group_id: Option<Value>,
}

#[derive(Debug, Deserialize)]
pub struct GpmGlobalGroup {
    pub id: Value,
    pub name: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Envelope<T> {
    data: Page<T>,
}

#[derive(Debug, Deserialize)]
struct Page<T> {
    data: Vec<T>,
    last_page: Option<i64>,
}

impl GpmGlobalClient {
    pub fn new(http: Client, base_url: String) -> Self {
        Self { http, base_url }
    }

    pub async fn list_profiles_page(&self, page: i64, page_size: i64) -> AppResult<(Vec<GpmGlobalProfile>, i64)> {
        let url = format!("{}/profiles", self.base_url.trim_end_matches('/'));
        let resp = self
            .http
            .get(url)
            .query(&[("page", page), ("per_page", page_size), ("page_size", page_size)])
            .send()
            .await?
            .error_for_status()?
            .json::<Envelope<GpmGlobalProfile>>()
            .await?;
        Ok((resp.data.data, resp.data.last_page.unwrap_or(1).max(1)))
    }

    pub async fn list_all_profiles(&self) -> AppResult<Vec<GpmGlobalProfile>> {
        let (mut all, total) = self.list_profiles_page(1, 100).await?;
        for page in 2..=total {
            let (mut rows, _) = self.list_profiles_page(page, 100).await?;
            all.append(&mut rows);
        }
        Ok(all)
    }

    pub async fn list_groups_page(&self, page: i64, page_size: i64) -> AppResult<(Vec<GpmGlobalGroup>, i64)> {
        let url = format!("{}/groups", self.base_url.trim_end_matches('/'));
        let resp = self
            .http
            .get(url)
            .query(&[("page", page), ("per_page", page_size), ("page_size", page_size)])
            .send()
            .await?
            .error_for_status()?
            .json::<Envelope<GpmGlobalGroup>>()
            .await?;
        Ok((resp.data.data, resp.data.last_page.unwrap_or(1).max(1)))
    }

    pub async fn list_groups(&self) -> AppResult<Vec<GpmGlobalGroup>> {
        let (mut all, total) = self.list_groups_page(1, 100).await?;
        for page in 2..=total {
            let (mut rows, _) = self.list_groups_page(page, 100).await?;
            all.append(&mut rows);
        }
        Ok(all)
    }
}

pub fn value_to_string(value: &Value) -> String {
    match value {
        Value::String(s) => s.clone(),
        Value::Number(n) => n.to_string(),
        other => other.to_string(),
    }
}
