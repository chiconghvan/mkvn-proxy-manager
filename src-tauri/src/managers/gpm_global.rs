use async_trait::async_trait;

use crate::{
    api::gpm_global::{value_to_string, GpmGlobalClient},
    error::AppResult,
    managers::ProfileManager,
    matcher::extract_host_port,
    models::{UnifiedGroup, UnifiedProfile},
};

pub struct GpmGlobalManager {
    client: GpmGlobalClient,
}

impl GpmGlobalManager {
    pub fn new(client: GpmGlobalClient) -> Self {
        Self { client }
    }
}

#[async_trait]
impl ProfileManager for GpmGlobalManager {
    fn name(&self) -> &'static str { "gpm_global" }
    fn display_name(&self) -> &'static str { "GPM Global" }

    async fn load_profiles(&self) -> AppResult<Vec<UnifiedProfile>> {
        let profiles = self.client.list_all_profiles().await?;
        Ok(profiles
            .into_iter()
            .map(|p| {
                let parsed = p.raw_proxy.as_deref().and_then(extract_host_port);
                UnifiedProfile {
                    id: value_to_string(&p.id),
                    name: p.name.unwrap_or_default(),
                    raw_proxy: p.raw_proxy,
                    host: parsed.as_ref().map(|(h, _)| h.clone()),
                    port: parsed.map(|(_, port)| port),
                    group_id: p.group_id.as_ref().map(value_to_string),
                    manager: self.name().to_string(),
                }
            })
            .collect())
    }

    async fn load_groups(&self) -> AppResult<Vec<UnifiedGroup>> {
        let groups = self.client.list_groups().await?;
        Ok(groups.into_iter().map(|g| UnifiedGroup {
            id: value_to_string(&g.id),
            name: g.name.unwrap_or_default(),
            manager: self.name().to_string(),
        }).collect())
    }
}
