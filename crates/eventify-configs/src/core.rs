use eventify_primitives::networks::{NetworkKind, ResourceKind};

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ManagerConfig {
    pub resources: std::collections::HashSet<ResourceKind>,
}

impl ManagerConfig {
    pub fn new(resources: std::collections::HashSet<ResourceKind>) -> Self {
        Self { resources }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
pub struct CollectorConfig {
    pub network: NetworkKind,
    pub client_url: String,
}

impl CollectorConfig {
    pub fn new(network: NetworkKind, client_url: String) -> Self {
        Self {
            network,
            client_url,
        }
    }
}
