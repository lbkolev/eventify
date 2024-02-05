use eventify_primitives::network::{NetworkKind, ResourceKind};

#[derive(Clone, Debug, Default)]
pub struct ManagerConfig {
    pub resources: std::collections::HashSet<ResourceKind>,
}

impl ManagerConfig {
    pub fn new(resources: std::collections::HashSet<ResourceKind>) -> Self {
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
