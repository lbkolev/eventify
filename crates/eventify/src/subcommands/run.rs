use std::{collections::HashSet, str::FromStr};

use alloy_primitives::BlockNumber;
use clap::{Args, Parser};
use secrecy::{ExposeSecret, Secret};

use eventify_configs::{configs::ServerConfig, Config, Mode, ModeKind, Network, NetworkDetail};
use eventify_primitives::{NetworkKind, ResourceKind};

#[derive(Clone, Debug, Parser)]
#[command(about = "Idx from range or stream directly from the tip of the chain")]
pub(crate) struct Cmd {
    #[arg(
        long = "config",
        env = "EVENTIFY_CONFIG",
        help = "Path to the config file",
        default_value = "etc/configs/default.toml",
        conflicts_with = "mode"
    )]
    pub(crate) config: Option<String>,

    #[arg(
        long = "mode",
        env = "EVENTIFY_MODE",
        help = "The mode to run the collector in",
        default_value_t = ModeKind::Stream,
        value_parser = ModeKind::from_str,
    )]
    pub(crate) mode: ModeKind,

    #[arg(
        long = "src-block",
        env = "EVENTIFY_SRC_BLOCK",
        help = "The block to begin the collecting from.",
        default_value = None
    )]
    pub(crate) src: Option<BlockNumber>,

    #[arg(
        long = "dst-block",
        env = "EVENTIFY_DST_BLOCK",
        help = "The block to end the collecting at.",
        default_value = None
    )]
    pub(crate) dst: Option<BlockNumber>,

    #[arg(
        long = "step",
        env = "EVENTIFY_STEP",
        help = "The step to use when collecting blocks.",
        default_value = None
    )]
    pub(crate) step: Option<BlockNumber>,

    #[arg(
        long,
        env = "DATABASE_URL",
        help = "The database URL to connect to",
        default_value = "postgres://postgres:password@localhost:5432/eventify"
    )]
    pub(crate) database_url: Secret<String>,

    #[arg(
        long,
        env = "REDIS_URL",
        help = "The redis URL to connect to",
        default_value = "redis://localhost:6379"
    )]
    pub(crate) redis_url: Secret<String>,

    #[arg(
        long,
        env = "EVENTIFY_NETWORK",
        help = "The type of network(s) to index",
        default_value_t = NetworkKind::Ethereum,
        value_parser = NetworkKind::from_str,
    )]
    pub(crate) network: NetworkKind,

    #[arg(
        long,
        env = "EVENTIFY_NODE_URL",
        help = "The node URL to connect to",
        default_value = "wss://eth.llamarpc.com"
    )]
    pub(crate) node_url: String,

    #[arg(
        long,
        env = "EVENTIFY_COLLECT",
        help = "Type of resources to collect",
        default_value = "blocks,tx,logs"
    )]
    pub(crate) collect: String,

    #[clap(flatten)]
    pub(crate) server: Option<ServerSettings>,
}

impl Cmd {
    pub(crate) fn collect(&self) -> HashSet<ResourceKind> {
        ResourceKind::resources_from_string(self.collect.clone())
    }

    pub(crate) fn server_host(&self) -> Option<String> {
        self.server.as_ref().map(|s| s.host.clone())
    }

    pub(crate) fn server_port(&self) -> Option<u16> {
        self.server.as_ref().map(|s| s.port)
    }

    pub(crate) fn database_url(&self) -> &str {
        self.database_url.expose_secret()
    }

    pub(crate) fn redis_url(&self) -> &str {
        self.redis_url.expose_secret()
    }

    pub(crate) fn node_url(&self) -> &str {
        &self.node_url
    }

    //    pub(crate) fn network(&self) -> NetworkKind {
    //        self.network
    //    }
}

impl From<Cmd> for Config {
    fn from(settings: Cmd) -> Self {
        Self {
            database_url: settings.database_url().to_string(),
            redis_url: settings.redis_url().to_string(),
            collect: settings.collect(),
            mode: Mode::new(settings.mode, settings.src, settings.dst, settings.step),
            server: Some(ServerConfig {
                host: settings.server_host().unwrap_or_default(),
                port: settings.server_port().unwrap_or_default(),
            }),
            network: Network {
                eth: Some(NetworkDetail {
                    node_url: settings.node_url().to_string(),
                }),
                zksync: None,
            },
            platform: None,
        }
    }
}

#[derive(Args, Clone, Debug)]
pub(crate) struct ServerSettings {
    #[arg(
        long = "server.threads",
        env = "EVENTIFY_SERVER_THREADS",
        help = "The number of threads to use for the API server",
        default_value_t = num_cpus::get(),
    )]
    pub(crate) threads: usize,

    #[arg(
        long = "server.host",
        env = "EVENTIFY_SERVER_HOST",
        help = "The host to run the HTTP-API server on",
        default_value = "127.0.0.1"
    )]
    pub(crate) host: String,

    #[arg(
        long = "server.port",
        env = "EVENTIFY_SERVER_PORT",
        help = "The port to run the HTTP-API server on",
        default_value_t = 21420,
        value_parser = clap::value_parser!(u16).range(1..),
    )]
    pub(crate) port: u16,
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::{Args, Parser};
    use std::env::{remove_var, set_var};

    // as env vars are global resource and tests by default are ran in parallel
    // we need to make sure that we run them in serial mode so they don't interfere with one another
    use serial_test::serial;

    // A helper type to parse Args more easily
    #[derive(Parser)]
    struct CommandParser<T: Args> {
        #[clap(flatten)]
        args: T,
    }

    #[test]
    #[serial]
    fn test_server_settings_default_values() {
        let args = CommandParser::<ServerSettings>::parse_from(["run"]).args;
        assert_eq!(args.host, "127.0.0.1");
        assert_eq!(args.port, 21420);
    }

    #[test]
    #[serial]
    fn test_server_settings_env_values() {
        set_var("EVENTIFY_SERVER_HOST", "localhost");
        set_var("EVENTIFY_SERVER_PORT", "1234");

        let args = CommandParser::<ServerSettings>::parse_from(["run"]).args;
        assert_eq!(args.host, "localhost");
        assert_eq!(args.port, 1234);

        remove_var("EVENTIFY_SERVER_HOST");
        remove_var("EVENTIFY_SERVER_PORT");
    }

    #[test]
    #[serial]
    fn test_server_settings_args_precedence() {
        set_var("EVENTIFY_SERVER_HOST", "localhost");
        set_var("EVENTIFY_SERVER_PORT", "1234");

        let args = CommandParser::<ServerSettings>::parse_from([
            "run",
            "--server.host",
            "1.2.3.4",
            "--server.port",
            "5678",
        ])
        .args;

        assert_eq!(args.host, "1.2.3.4");

        remove_var("EVENTIFY_SERVER_HOST");
        remove_var("EVENTIFY_SERVER_PORT");
    }
}
