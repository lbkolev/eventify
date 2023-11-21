use alloy_primitives::BlockNumber;
use clap::Parser;
use secrecy::{ExposeSecret, Secret};

use chainthru_primitives as types;
use chainthru_server as server;

#[derive(Debug, clap::Args, Clone)]
#[group(skip)]
pub struct IndexerSettings {
    #[arg(
        long = "indexer.enabled",
        env = "CHAINTHRU_INDEXER_ENABLED",
        help = "Toggler enabling|disabling the indexer",
        action
    )]
    pub indexer_enabled: bool,

    #[arg(
        long,
        env = "CHAINTHRU_SRC_BLOCK",
        help = "The block to begin the indexing from. Defaults to 0",
        default_value_t = 0
    )]
    pub src_block: BlockNumber,

    #[arg(
        long,
        env = "CHAINTHRU_DST_BLOCK",
        help = "The block to end the indexing at. Defaults to the latest block",
        default_value_t = BlockNumber::MAX
    )]
    pub dst_block: BlockNumber,
}

#[derive(Debug, clap::Args, Clone)]
#[group(skip)]
pub struct ServerSettings {
    #[arg(
        long = "server.enabled",
        env = "CHAINTHRU_SERVER_ENABLED",
        help = "Toggler enabling|disabling the HTTP-API server",
        action
    )]
    pub server_enabled: bool,

    #[arg(
        long = "server.threads",
        env = "CHAINTHRU_SERVER_THREADS",
        help = "The number of threads to use for the API server",
        default_value_t = num_cpus::get(),
    )]
    pub server_threads: usize,

    #[arg(
        long = "server.host",
        env = "CHAINTHRU_SERVER_HOST",
        help = "The host to run the HTTP-API server on",
        default_value = ""
    )]
    pub host: String,

    #[arg(
        long = "server.port",
        env = "CHAINTHRU_SERVER_PORT",
        help = "The port to run the HTTP-API server on",
        default_value_t = 6969,
        value_parser = clap::value_parser!(u16).range(1..),
    )]
    pub port: u16,
}

#[derive(Clone, Debug, Parser)]
#[command(name = "Chainthru")]
#[command(about = "Index Ethereum into a Postgresql database & serve it via an API server.")]
pub struct Settings {
    #[clap(flatten)]
    pub indexer: IndexerSettings,

    #[clap(flatten)]
    pub server: ServerSettings,

    #[arg(
        long,
        env = "CHAINTHRU_STORAGE_URL",
        help = "The database URL to connect to",
        default_value = "postgres://postgres:password@localhost:5432/chainthru"
    )]
    pub storage_url: Secret<String>,

    #[arg(
        long,
        env = "CHAINTHRU_NODE_URL",
        help = "The Ethereum node URL to connect to",
        default_value = "wss://eth.llamarpc.com"
    )]
    pub node_url: String,

    #[arg(
        long = "log.level",
        env = "RUST_LOG",
        help = "The log level to use",
        default_value = "warn",
        value_parser = clap::value_parser!(log::Level),
    )]
    pub log_level: log::Level,
}

impl From<Settings> for server::Settings {
    fn from(settings: Settings) -> Self {
        Self {
            database: types::DatabaseSettings::from(settings.storage_url.expose_secret().clone()),
            application: server::ApplicationSettings {
                host: settings.server.host,
                port: settings.server.port,
                worker_threads: settings.server.server_threads,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Args;

    // as env vars are global resource and tests by default are ran in parallel
    // we need to make sure that we run them in serial mode so they don't interfere with one another
    use serial_test::serial;

    #[derive(Parser)]
    struct CommandParser<T: Args> {
        #[clap(flatten)]
        args: T,
    }

    #[test]
    #[serial]
    fn test_indexer_settings_default_values() {
        let args = CommandParser::<IndexerSettings>::parse_from(["chainthru"]).args;
        assert!(!args.indexer_enabled);
        assert_eq!(args.src_block, 0);
        assert_eq!(args.dst_block, BlockNumber::MAX);
    }

    #[test]
    #[serial]
    fn test_indexer_settings_env_values() {
        std::env::set_var("CHAINTHRU_INDEXER_ENABLED", "true");
        std::env::set_var("CHAINTHRU_SRC_BLOCK", "1");
        std::env::set_var("CHAINTHRU_DST_BLOCK", "2");

        let args = CommandParser::<IndexerSettings>::parse_from(["chainthru"]).args;
        assert!(args.indexer_enabled);
        assert_eq!(args.src_block, 1);
        assert_eq!(args.dst_block, 2);

        std::env::remove_var("CHAINTHRU_INDEXER_ENABLED");
        std::env::remove_var("CHAINTHRU_SRC_BLOCK");
        std::env::remove_var("CHAINTHRU_DST_BLOCK");
    }

    #[test]
    #[serial]
    fn test_indexer_settings_args_precedence() {
        std::env::set_var("CHAINTHRU_SRC_BLOCK", "1");
        std::env::set_var("CHAINTHRU_DST_BLOCK", "2");

        let args = CommandParser::<IndexerSettings>::parse_from([
            "chainthru",
            "--indexer.enabled",
            "--src-block",
            "3",
            "--dst-block",
            "4",
        ])
        .args;
        assert!(args.indexer_enabled);
        assert_eq!(args.src_block, 3);
        assert_eq!(args.dst_block, 4);

        std::env::remove_var("CHAINTHRU_INDEXER_ENABLED");
        std::env::remove_var("CHAINTHRU_SRC_BLOCK");
        std::env::remove_var("CHAINTHRU_DST_BLOCK");
    }

    #[test]
    #[serial]
    fn test_server_settings_default_values() {
        let args = CommandParser::<ServerSettings>::parse_from(["chainthru"]).args;
        assert!(!args.server_enabled);
        assert_eq!(args.server_threads, num_cpus::get());
        assert_eq!(args.host, "");
        assert_eq!(args.port, 6969);
    }

    #[test]
    #[serial]
    fn test_server_settings_env_values() {
        std::env::set_var("CHAINTHRU_SERVER_ENABLED", "true");
        std::env::set_var("CHAINTHRU_SERVER_THREADS", "1");
        std::env::set_var("CHAINTHRU_SERVER_HOST", "localhost");
        std::env::set_var("CHAINTHRU_SERVER_PORT", "1234");

        let args = CommandParser::<ServerSettings>::parse_from(["chainthru"]).args;
        assert!(args.server_enabled);
        assert_eq!(args.server_threads, 1);
        assert_eq!(args.host, "localhost");
        assert_eq!(args.port, 1234);

        std::env::remove_var("CHAINTHRU_SERVER_ENABLED");
        std::env::remove_var("CHAINTHRU_SERVER_THREADS");
        std::env::remove_var("CHAINTHRU_SERVER_HOST");
        std::env::remove_var("CHAINTHRU_SERVER_PORT");
    }

    #[test]
    #[serial]
    fn test_server_settings_args_precedence() {
        std::env::set_var("CHAINTHRU_SERVER_THREADS", "1");
        std::env::set_var("CHAINTHRU_SERVER_HOST", "localhost");
        std::env::set_var("CHAINTHRU_SERVER_PORT", "1234");

        let args = CommandParser::<ServerSettings>::parse_from([
            "chainthru",
            "--server.enabled",
            "--server.threads",
            "2",
            "--server.host",
            "1.2.3.4",
            "--server.port",
            "5678",
        ])
        .args;

        assert!(args.server_enabled);
        assert_eq!(args.server_threads, 2);
        assert_eq!(args.host, "1.2.3.4");

        std::env::remove_var("CHAINTHRU_SERVER_ENABLED");
        std::env::remove_var("CHAINTHRU_SERVER_THREADS");
        std::env::remove_var("CHAINTHRU_SERVER_HOST");
        std::env::remove_var("CHAINTHRU_SERVER_PORT");
    }
}
