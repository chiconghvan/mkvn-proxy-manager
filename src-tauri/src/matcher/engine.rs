use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::models::{UnifiedGroup, UnifiedProfile};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileMatch {
    pub manager: String,
    pub profile_id: String,
    pub profile_name: String,
    pub group_id: Option<String>,
    pub group_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchResult {
    pub proxy_host: String,
    pub proxy_port: u16,
    pub order_code: String,
    pub manager: Option<String>,
    pub profile_id: Option<String>,
    pub profile_name: Option<String>,
    pub group_name: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct ProxyProfileMatcher {
    map: HashMap<String, Vec<ProfileMatch>>,
}

impl ProxyProfileMatcher {
    pub fn build(profiles: &[UnifiedProfile], groups: &[UnifiedGroup]) -> Self {
        let group_map: HashMap<(String, String), String> = groups
            .iter()
            .map(|g| ((g.manager.clone(), g.id.clone()), g.name.clone()))
            .collect();
        let mut map: HashMap<String, Vec<ProfileMatch>> = HashMap::new();

        for profile in profiles {
            let (Some(host), Some(port)) = (&profile.host, profile.port) else { continue; };
            let key = format!("{}:{}", host.to_ascii_lowercase(), port);
            let group_name = profile
                .group_id
                .as_ref()
                .and_then(|id| group_map.get(&(profile.manager.clone(), id.clone())).cloned());
            map.entry(key).or_default().push(ProfileMatch {
                manager: profile.manager.clone(),
                profile_id: profile.id.clone(),
                profile_name: profile.name.clone(),
                group_id: profile.group_id.clone(),
                group_name,
            });
        }

        Self { map }
    }

    pub fn match_proxy(&self, host: &str, port: u16) -> Option<&Vec<ProfileMatch>> {
        let key = format!("{}:{}", host.to_ascii_lowercase(), port);
        self.map.get(&key)
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }
}
