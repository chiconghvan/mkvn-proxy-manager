use std::sync::Arc;

use crate::managers::ProfileManager;

#[derive(Clone, Default)]
pub struct ManagerRegistry {
    managers: Vec<Arc<dyn ProfileManager>>,
}

impl ManagerRegistry {
    pub fn new() -> Self {
        Self { managers: Vec::new() }
    }

    pub fn register(&mut self, manager: Arc<dyn ProfileManager>) {
        self.managers.push(manager);
    }

    pub fn all(&self) -> Vec<Arc<dyn ProfileManager>> {
        self.managers.clone()
    }

    pub fn is_empty(&self) -> bool {
        self.managers.is_empty()
    }
}
