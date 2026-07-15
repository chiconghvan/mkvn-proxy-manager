use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedGroup {
    pub id: String,
    pub name: String,
    pub manager: String,
}
