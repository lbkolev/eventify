pub mod core;
pub mod database;
pub mod server;

pub mod configs {
    pub use crate::{
        core::{CollectorConfig, ManagerConfig},
        database::DatabaseConfig,
        server::{ApplicationConfig, ServerConfig},
    };
}

use std::collections::HashSet;

use eventify_primitives::networks::{LogKind, ResourceKind};

pub fn deserialize_resource_kinds<'de, D>(
    deserializer: D,
) -> Result<HashSet<ResourceKind>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: String = serde::Deserialize::deserialize(deserializer)?;
    s.split(',')
        .map(|x| match x.trim().to_lowercase().as_str() {
            "block" | "blocks" => Ok(ResourceKind::Block),
            "tx" | "txs" | "transactions" => Ok(ResourceKind::Transaction),
            "log" | "logs" => Ok(ResourceKind::Log(LogKind::Raw)),
            other => Err(serde::de::Error::custom(format!(
                "unknown resource kind: {}",
                other
            ))),
        })
        .collect()
}

#[derive(Debug, serde::Deserialize, Default)]
pub struct Config {
    pub database_url: String,
    pub queue_url: String,

    #[serde(deserialize_with = "deserialize_resource_kinds")]
    pub collect: HashSet<ResourceKind>,
    pub server: Option<crate::configs::ServerConfig>,
    pub network: Option<Network>,
}

impl Config {
    pub fn new(
        database_url: String,
        queue_url: String,
        collect: HashSet<ResourceKind>,
        server: Option<crate::configs::ServerConfig>,
        network: Option<Network>,
    ) -> Self {
        Self {
            database_url,
            queue_url,
            collect,
            server,
            network,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, Default)]
pub struct Network {
    pub eth: Option<NetworkDetail>,
    pub zksync: Option<NetworkDetail>,
}

impl Network {
    pub fn new(eth: Option<NetworkDetail>, zksync: Option<NetworkDetail>) -> Self {
        Self { eth, zksync }
    }
}

#[derive(Clone, Debug, serde::Deserialize, Default)]
pub struct NetworkDetail {
    pub node_url: String,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parses_config_correctly() {
        const TEST_TOML: &str = r#"
database_url = "postgres://postgres:password@localhost:5432/eventify"
queue_url = "redis://localhost:6379"
collect = "blocks,tx,logs"

[server]
host = "0.0.0.0"
port = 21420

[network]
    [network.eth]
    node_url = "wss://eth.llamarpc.com"

    [network.zksync]
    node_url = "wss://mainnet.era.zksync.io/ws"
"#;
        let config: Config = toml::from_str(TEST_TOML).expect("Failed to parse TOML");

        assert_eq!(
            config.database_url,
            "postgres://postgres:password@localhost:5432/eventify"
        );
        assert_eq!(config.queue_url, "redis://localhost:6379");
        assert_eq!(
            config.collect,
            HashSet::from([
                ResourceKind::Block,
                ResourceKind::Log(LogKind::Raw),
                ResourceKind::Transaction
            ])
        );
        assert_eq!(config.server.unwrap().port, 21420);
        assert_eq!(
            config
                .network
                .as_ref()
                .and_then(|network| network.eth.as_ref())
                .map(|eth| &eth.node_url),
            Some(&"wss://eth.llamarpc.com".to_string())
        );
        assert_eq!(
            config
                .network
                .as_ref()
                .and_then(|network| network.zksync.as_ref())
                .map(|zksync| &zksync.node_url),
            Some(&"wss://mainnet.era.zksync.io/ws".to_string())
        );
    }

    #[test]
    fn test_parses_config_without_platform_correctly() {
        const TEST_TOML: &str = r#"
database_url = "postgres://postgres:password@localhost:5432/eventify"
queue_url = "redis://localhost:6379"
collect = "logs,tx"

[server]
host = "0.0.0.0"
port = 21420

[network]
    [network.eth]
    node_url = "wss://eth.llamarpc.com"
"#;

        let config: Config = toml::from_str(TEST_TOML).expect("Failed to parse TOML");

        assert_eq!(
            config.database_url,
            "postgres://postgres:password@localhost:5432/eventify"
        );
        assert_eq!(config.queue_url, "redis://localhost:6379");
        assert_eq!(
            config.collect,
            HashSet::from([ResourceKind::Log(LogKind::Raw), ResourceKind::Transaction,])
        );
        assert_eq!(config.server.unwrap().port, 21420);
        assert_eq!(
            config
                .network
                .as_ref()
                .and_then(|network| network.eth.as_ref())
                .map(|eth| &eth.node_url),
            Some(&"wss://eth.llamarpc.com".to_string())
        );
        assert!(config.network.unwrap().zksync.is_none());
    }
}
