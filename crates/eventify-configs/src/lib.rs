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

use std::{collections::HashSet, fmt, str::FromStr};

use eventify_primitives::{LogKind, ResourceKind};
use serde::{self, Deserialize, Deserializer};
use server::ServerConfig;

#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "lowercase")]
pub enum ModeKind {
    #[default]
    Stream,
    Batch,
}

impl FromStr for ModeKind {
    type Err = serde::de::value::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "stream" => Ok(ModeKind::Stream),
            "batch" => Ok(ModeKind::Batch),
            _ => Err(serde::de::Error::custom(format!("unknown mode: {}", s))),
        }
    }
}

impl fmt::Display for ModeKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ModeKind::Stream => write!(f, "stream"),
            ModeKind::Batch => write!(f, "batch"),
        }
    }
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct Mode {
    pub kind: ModeKind,

    pub src: Option<u64>,
    pub dst: Option<u64>,
    pub step: Option<u64>,
}

impl Default for Mode {
    fn default() -> Self {
        Self::default_stream()
    }
}

impl Mode {
    pub fn new(kind: ModeKind, src: Option<u64>, dst: Option<u64>, step: Option<u64>) -> Self {
        Self {
            kind,
            src,
            dst,
            step,
        }
    }

    pub fn default_from_kind(kind: ModeKind) -> Mode {
        match kind {
            ModeKind::Stream => Self::default_stream(),
            ModeKind::Batch => Self::default_batch(),
        }
    }

    pub fn default_stream() -> Self {
        Self {
            kind: ModeKind::Stream,
            src: None,
            dst: None,
            step: None,
        }
    }

    pub fn default_batch() -> Self {
        Self {
            kind: ModeKind::Batch,
            src: None,
            dst: None,
            step: Some(1),
        }
    }
}

// Helper function for deserializing a comma-separated string into a Vec<ResourceKind>
pub fn deserialize_resource_kinds<'de, D>(
    deserializer: D,
) -> Result<HashSet<ResourceKind>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
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

#[derive(Debug, Deserialize, Default)]
pub struct Config {
    pub database_url: String,
    pub redis_url: String,

    #[serde(deserialize_with = "deserialize_resource_kinds")]
    pub collect: HashSet<ResourceKind>,
    pub mode: Mode,
    pub server: Option<ServerConfig>,
    pub network: Network,
    pub platform: Option<Platform>,
}

impl Config {
    pub fn new(
        database_url: String,
        redis_url: String,
        collect: HashSet<ResourceKind>,
        mode: Mode,
        server: Option<ServerConfig>,
        network: Network,
        platform: Option<Platform>,
    ) -> Self {
        Self {
            database_url,
            redis_url,
            collect,
            mode,
            server,
            network,
            platform,
        }
    }
}

#[derive(Debug, Deserialize, Default)]
pub struct Network {
    pub eth: Option<NetworkDetail>,
    pub zksync: Option<NetworkDetail>,
}

#[derive(Debug, Deserialize, Default)]
pub struct NetworkDetail {
    pub node_url: String,
}

#[derive(Debug, Deserialize, Default)]
pub struct Platform {
    pub discord: Option<PlatformDetail>,
    pub slack: Option<PlatformDetail>,
}

#[derive(Debug, Deserialize, Default)]
pub struct PlatformDetail {
    pub token: String,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parses_config_with_stream_mode_correctly() {
        const TEST_TOML: &str = r#"
database_url = "postgres://postgres:password@localhost:5432/eventify"
redis_url = "redis://localhost:6379"
collect = "blocks,tx,logs"

[server]
port = 21420

[mode]
type = "stream"

[network]
    [network.eth]
    node_url = "wss://eth.llamarpc.com"

    [network.zksync]
    node_url = "wss://mainnet.era.zksync.io/ws"

[platform]
    [platform.discord]
    token = "discord_token"

    [platform.slack]
    token = "slack_token"
"#;
        let config: Config = toml::from_str(TEST_TOML).expect("Failed to parse TOML");

        assert_eq!(
            config.database_url,
            "postgres://postgres:password@localhost:5432/eventify"
        );
        assert_eq!(config.redis_url, "redis://localhost:6379");
        assert_eq!(
            config.collect,
            HashSet::from([
                ResourceKind::Block,
                ResourceKind::Log(LogKind::Raw),
                ResourceKind::Transaction
            ])
        );
        assert_eq!(config.mode.kind, ModeKind::Stream);
        assert_eq!(config.mode.src, None);
        assert_eq!(config.mode.dst, None);
        assert_eq!(config.mode.step, None);
        assert_eq!(config.server.unwrap().port, 21420);
        assert_eq!(
            config.network.eth.unwrap().node_url,
            "wss://eth.llamarpc.com"
        );
        assert_eq!(
            config.network.zksync.unwrap().node_url,
            "wss://mainnet.era.zksync.io/ws"
        );
        //assert_eq!(
        //    config.platform.unwrap().discord.unwrap().token,
        //    "discord_token"
        //);
        //assert_eq!(config.platform.unwrap().slack.unwrap().token, "slack_token");
    }

    #[test]
    fn test_parses_config_with_batch_mode_correctly() {
        const TEST_TOML: &str = r#"
database_url = "postgres://postgres:password@localhost:5432/eventify"
redis_url = "redis://localhost:6379"
collect = "blocks"

[mode]
type = "batch"
src = 1
dst = 15000
step = 1

[server]
port = 21420

[network]
    [network.eth]
    node_url = "wss://eth.llamarpc.com"
[platform]
    [platform.discord]
    token = "rand"
"#;

        let config: Config = toml::from_str(TEST_TOML).expect("Failed to parse TOML");

        assert_eq!(
            config.database_url,
            "postgres://postgres:password@localhost:5432/eventify"
        );
        assert_eq!(config.redis_url, "redis://localhost:6379");
        assert!(config.collect.contains(&ResourceKind::Block));
        assert_eq!(config.mode.kind, ModeKind::Batch);
        assert_eq!(config.mode.src, Some(1));
        assert_eq!(config.mode.dst, Some(2));
        assert_eq!(config.mode.step, Some(1));
        assert_eq!(config.server.unwrap().port, 21420);
        assert_eq!(
            config.network.eth.unwrap().node_url,
            "wss://eth.llamarpc.com"
        );
        assert!(config.network.zksync.is_none());
        assert_eq!(config.platform.unwrap().discord.unwrap().token, "rand");
    }

    #[test]
    fn test_parses_config_without_platform_correctly() {
        const TEST_TOML: &str = r#"
database_url = "postgres://postgres:password@localhost:5432/eventify"
redis_url = "redis://localhost:6379"
collect = "logs,tx"

[server]
port = 21420

[mode]
type = "stream"

[network]
    [network.eth]
    node_url = "wss://eth.llamarpc.com"
"#;

        let config: Config = toml::from_str(TEST_TOML).expect("Failed to parse TOML");

        assert_eq!(
            config.database_url,
            "postgres://postgres:password@localhost:5432/eventify"
        );
        assert_eq!(config.redis_url, "redis://localhost:6379");
        assert_eq!(
            config.collect,
            HashSet::from([ResourceKind::Log(LogKind::Raw), ResourceKind::Transaction,])
        );
        assert_eq!(config.mode.kind, ModeKind::Stream);
        assert_eq!(config.server.unwrap().port, 21420);
        assert_eq!(
            config.network.eth.unwrap().node_url,
            "wss://eth.llamarpc.com"
        );
        assert!(config.network.zksync.is_none());
        assert!(config.platform.is_none());
    }
}
