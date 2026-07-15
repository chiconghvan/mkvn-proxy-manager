use async_trait::async_trait;

use crate::{
    api::donut::DonutClient,
    error::AppResult,
    managers::ProfileManager,
    matcher::extract_host_port,
    models::{UnifiedGroup, UnifiedProfile},
};

pub struct DonutManager {
    client: DonutClient,
}

impl DonutManager {
    pub fn new(client: DonutClient) -> Self {
        Self { client }
    }
}

#[async_trait]
impl ProfileManager for DonutManager {
    fn name(&self) -> &'static str { "donut" }
    fn display_name(&self) -> &'static str { "Donut Browser" }

    async fn load_profiles(&self) -> AppResult<Vec<UnifiedProfile>> {
        let profiles = self.client.list_profiles().await?;
        Ok(profiles.into_iter().map(|p| {
            let parsed = p.proxy.as_deref().and_then(extract_host_port);
            UnifiedProfile {
                id: p.id,
                name: p.name.unwrap_or_default(),
                raw_proxy: p.proxy,
                host: parsed.as_ref().map(|(h, _)| h.clone()),
                port: parsed.map(|(_, port)| port),
                group_id: p.group_id,
                manager: self.name().to_string(),
            }
        }).collect())
    }

    async fn load_groups(&self) -> AppResult<Vec<UnifiedGroup>> {
        let groups = self.client.list_groups().await?;
        Ok(groups.into_iter().map(|g| UnifiedGroup {
            id: g.id,
            name: g.name.unwrap_or_default(),
            manager: self.name().to_string(),
        }).collect())
    }
}
