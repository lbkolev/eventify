use std::{collections::HashSet, str::FromStr};

use clap::{Args, Parser};
use secrecy::{ExposeSecret, Secret};

use eventify_configs::{configs::ServerConfig, Config, Network, NetworkDetail};
use eventify_primitives::networks::{NetworkKind, ResourceKind};

#[derive(Clone, Debug, Parser)]
#[command(about = "Idx from range or stream directly from the tip of the chain")]
pub(crate) struct Cmd {
    #[arg(
        long,
        env = "EVENTIFY_CONFIG",
        help = "Path to the config file",
        default_value = None,
    )]
    pub(crate) config: Option<String>,

    #[arg(
        long,
        env = "DATABASE_URL",
        help = "The database URL to connect to",
        default_value = "postgres://postgres:password@localhost:5432/eventify"
    )]
    pub(crate) database_url: Secret<String>,

    #[arg(
        long,
        env = "QUEUE_URL",
        help = "The redis URL to connect to",
        default_value = "redis://localhost:6379"
    )]
    pub(crate) queue_url: Secret<String>,

    #[arg(
        long,
        env = "EVENTIFY_NETWORK",
        help = "The type of network to stream from",
        default_value_t = NetworkKind::Ethereum,
        value_parser = NetworkKind::from_str,
    )]
    pub(crate) network: NetworkKind,

    #[arg(
        long,
        env = "EVENTIFY_NODE_URL",
        help = "The network node URL to connect to",
        default_value = "wss://eth.llamarpc.com"
    )]
    pub(crate) node_url: String,

    #[arg(
        long,
        env = "EVENTIFY_NOTIFY",
        help = "Notify toggler, enabled means notify for all present notifications",
        default_value = "true",
        value_parser = |s: &str| s.parse::<bool>(),
    )]
    pub(crate) notify: bool,

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

    pub(crate) fn database_url(&self) -> &str {
        self.database_url.expose_secret()
    }

    pub(crate) fn queue_url(&self) -> &str {
        self.queue_url.expose_secret()
    }

    pub(crate) fn node_url(&self) -> &str {
        &self.node_url
    }

    pub(crate) fn notify(&self) -> bool {
        self.notify
    }
}

impl From<Cmd> for Config {
    fn from(settings: Cmd) -> Self {
        let server = match settings.server.clone() {
            Some(s) => Some(ServerConfig {
                host: s.host,
                port: s.port,
            }),
            None => None,
        };

        let network = match settings.network {
            NetworkKind::Ethereum => Some(Network {
                eth: Some(NetworkDetail {
                    node_url: settings.clone().node_url().to_string(),
                }),
                zksync: None,
            }),
            NetworkKind::Zksync => Some(Network {
                eth: None,
                zksync: Some(NetworkDetail {
                    node_url: settings.clone().node_url().to_string(),
                }),
            }),
        };

        Self {
            database_url: settings.database_url().to_string(),
            queue_url: settings.queue_url().to_string(),
            collect: settings.collect(),
            notify: settings.notify(),
            server,
            network,
        }
    }
}

#[derive(Args, Clone, Debug, Eq, PartialEq)]
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
            "--server.host=1.2.3.4",
            "--server.port=5678",
        ])
        .args;

        assert_eq!(args.host, "1.2.3.4");
        assert_eq!(args.port, 5678);

        remove_var("EVENTIFY_SERVER_HOST");
        remove_var("EVENTIFY_SERVER_PORT");
    }

    #[test]
    #[serial]
    fn test_run_default_values() {
        let args = CommandParser::<Cmd>::parse_from(["run"]).args;

        assert_eq!(args.config, None);
        assert_eq!(
            args.database_url(),
            "postgres://postgres:password@localhost:5432/eventify"
        );
        assert_eq!(args.queue_url(), "redis://localhost:6379");
        assert_eq!(args.network, NetworkKind::Ethereum);
        assert_eq!(args.node_url(), "wss://eth.llamarpc.com");
        assert_eq!(args.collect, "blocks,tx,logs");
        assert_eq!(args.server, None);
    }

    #[test]
    #[serial]
    fn test_run_env_values() {
        set_var(
            "DATABASE_URL",
            "postgres://postgres:xxxxxxxx@xxxxxxxxx:5432/eventify",
        );
        set_var("EVENTIFY_QUEUE_URL", "redis://localhost:6379");
        set_var("EVENTIFY_NETWORK", "zksync");
        set_var("EVENTIFY_NODE_URL", "wss://zksync.llamarpc.com");
        set_var("EVENTIFY_COLLECT", "txs,logs");
        set_var("EVENTIFY_SERVER_HOST", "127.0.0.1");
        set_var("EVENTIFY_SERVER_PORT", "1234");

        let args = CommandParser::<Cmd>::parse_from(["run"]).args;

        assert_eq!(
            args.database_url(),
            "postgres://postgres:xxxxxxxx@xxxxxxxxx:5432/eventify"
        );
        assert_eq!(args.queue_url(), "redis://localhost:6379");
        assert_eq!(args.network, NetworkKind::Zksync);
        assert_eq!(args.node_url(), "wss://zksync.llamarpc.com");
        assert_eq!(args.collect, "txs,logs");
        assert_eq!(
            args.server,
            Some(ServerSettings {
                threads: num_cpus::get(),
                host: "127.0.0.1".to_string(),
                port: 1234
            })
        );

        remove_var("EVENTIFY_CONFIG");
        remove_var("DATABASE_URL");
        remove_var("EVENTIFY_QUEUE_URL");
        remove_var("EVENTIFY_NETWORK");
        remove_var("EVENTIFY_NODE_URL");
        remove_var("EVENTIFY_COLLECT");
        remove_var("EVENTIFY_SERVER_HOST");
        remove_var("EVENTIFY_SERVER_PORT");
    }

    #[test]
    #[serial]
    fn test_run_args_precedence() {
        set_var(
            "DATABASE_URL",
            "postgres://postgres:xxxxxxxx@xxxxxxxxx:5432/eventify",
        );
        set_var("EVENTIFY_QUEUE_URL", "redis://localhost:6379");
        set_var("EVENTIFY_NETWORK", "zksync");
        set_var("EVENTIFY_NODE_URL", "wss://zksync.llamarpc.com");
        set_var("EVENTIFY_COLLECT", "txs,logs");
        set_var("EVENTIFY_SERVER_HOST", "localhost");
        set_var("EVENTIFY_SERVER_PORT", "1234");

        let args = CommandParser::<Cmd>::parse_from([
            "run",
            "--database-url=postgres://postgres:xxxxxxxx@xxxxxxxxx:5432/eventify",
            "--queue-url=redis://localhost:6379",
            "--network=ethereum",
            "--node-url=wss://eth.llamarpc.com",
            "--collect=txs,logs,blocks",
            "--server.host=localhost",
        ])
        .args;

        assert_eq!(
            args.database_url(),
            "postgres://postgres:xxxxxxxx@xxxxxxxxx:5432/eventify"
        );
        assert_eq!(args.queue_url(), "redis://localhost:6379");
        assert_eq!(args.network, NetworkKind::Ethereum);
        assert_eq!(args.node_url(), "wss://eth.llamarpc.com");
        assert_eq!(args.collect, "txs,logs,blocks");
        assert_eq!(
            args.server,
            Some(ServerSettings {
                threads: num_cpus::get(),
                host: "localhost".to_string(),
                port: 1234
            })
        );

        remove_var("DATABASE_URL");
        remove_var("EVENTIFY_QUEUE_URL");
        remove_var("EVENTIFY_NETWORK");
        remove_var("EVENTIFY_NODE_URL");
        remove_var("EVENTIFY_COLLECT");
        remove_var("EVENTIFY_SERVER_HOST");
        remove_var("EVENTIFY_SERVER_PORT");
    }
}
