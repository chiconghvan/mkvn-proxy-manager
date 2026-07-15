use reqwest::Client;
use serde::Deserialize;
use serde_json::Value;

use crate::error::AppResult;

#[derive(Clone)]
pub struct GpmStandardClient {
    http: Client,
    base_url: String,
}

#[derive(Debug, Deserialize)]
pub struct GpmStandardProfile {
    pub id: Value,
    pub name: Option<String>,
    pub raw_proxy: Option<String>,
    pub group_id: Option<Value>,
}

#[derive(Debug, Deserialize)]
pub struct GpmStandardGroup {
    pub id: Value,
    pub name: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ProfilesResponse {
    data: Vec<GpmStandardProfile>,
    pagination: Option<Pagination>,
}

#[derive(Debug, Deserialize)]
struct Pagination {
    total_page: Option<i64>,
    #[allow(dead_code)]
    current_page: Option<i64>,
}

#[derive(Debug, Deserialize)]
struct GroupsResponse {
    data: Vec<GpmStandardGroup>,
}

impl GpmStandardClient {
    pub fn new(http: Client, base_url: String) -> Self {
        Self { http, base_url }
    }

    pub async fn list_profiles_page(&self, page: i64, per_page: i64) -> AppResult<(Vec<GpmStandardProfile>, i64)> {
        let url = format!("{}/profiles", self.base_url.trim_end_matches('/'));
        let resp = self
            .http
            .get(url)
            .query(&[("page", page), ("per_page", per_page)])
            .send()
            .await?
            .error_for_status()?
            .json::<ProfilesResponse>()
            .await?;
        let total = resp.pagination.and_then(|p| p.total_page).unwrap_or(1).max(1);
        Ok((resp.data, total))
    }

    pub async fn list_all_profiles(&self) -> AppResult<Vec<GpmStandardProfile>> {
        let (mut all, total) = self.list_profiles_page(1, 100).await?;
        for page in 2..=total {
            let (mut rows, _) = self.list_profiles_page(page, 100).await?;
            all.append(&mut rows);
        }
        Ok(all)
    }

    pub async fn list_groups(&self) -> AppResult<Vec<GpmStandardGroup>> {
        let url = format!("{}/groups", self.base_url.trim_end_matches('/'));
        let resp = self.http.get(url).send().await?.error_for_status()?.json::<GroupsResponse>().await?;
        Ok(resp.data)
    }
}

pub fn value_to_string(value: &Value) -> String {
    match value {
        Value::String(s) => s.clone(),
        Value::Number(n) => n.to_string(),
        other => other.to_string(),
    }
}
