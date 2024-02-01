use std::collections::HashSet;

use eventify_primitives::{NetworkKind, ResourceKind};

#[derive(Clone, Debug, Default)]
pub struct ManagerConfig {
    pub resources: HashSet<ResourceKind>,
}

impl ManagerConfig {
    pub fn new(resources: HashSet<ResourceKind>) -> Self {
        Self { resources }
    }
}

#[derive(Clone, Debug, Default)]
pub struct CollectorConfig {
    pub network: NetworkKind,
}

impl CollectorConfig {
    pub fn new(network: NetworkKind) -> Self {
        Self { network }
    }
}
