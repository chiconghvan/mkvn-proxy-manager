use reqwest::Client;
use serde::Deserialize;

use crate::error::AppResult;

#[derive(Clone)]
pub struct DonutClient {
    http: Client,
    base_url: String,
}

#[derive(Debug, Deserialize)]
pub struct DonutProfile {
    pub id: String,
    pub name: Option<String>,
    pub proxy: Option<String>,
    pub group_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DonutGroup {
    pub id: String,
    pub name: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ProfilesResponse {
    profiles: Vec<DonutProfile>,
}

impl DonutClient {
    pub fn new(http: Client, base_url: String) -> Self {
        Self { http, base_url }
    }

    pub async fn list_profiles(&self) -> AppResult<Vec<DonutProfile>> {
        let url = format!("{}/profiles", self.base_url.trim_end_matches('/'));
        let resp = self.http.get(url).send().await?.error_for_status()?.json::<ProfilesResponse>().await?;
        Ok(resp.profiles)
    }

    pub async fn list_groups(&self) -> AppResult<Vec<DonutGroup>> {
        let url = format!("{}/groups", self.base_url.trim_end_matches('/'));
        let resp = self.http.get(url).send().await?.error_for_status()?.json::<Vec<DonutGroup>>().await?;
        Ok(resp)
    }
}
