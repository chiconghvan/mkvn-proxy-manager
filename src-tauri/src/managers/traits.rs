use async_trait::async_trait;

use crate::{error::AppResult, models::{UnifiedGroup, UnifiedProfile}};

#[async_trait]
pub trait ProfileManager: Send + Sync {
    fn name(&self) -> &'static str;
    fn display_name(&self) -> &'static str;
    async fn load_profiles(&self) -> AppResult<Vec<UnifiedProfile>>;
    async fn load_groups(&self) -> AppResult<Vec<UnifiedGroup>>;
}
