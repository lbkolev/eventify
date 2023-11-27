use alloy_primitives::BlockNumber;
use clap::Parser;
use secrecy::{ExposeSecret, Secret};

use chainthru_primitives as types;
use chainthru_server as server;

#[derive(Debug, clap::Args, Clone)]
pub(crate) struct Block {
    #[arg(
        long = "src-block",
        env = "CHAINTHRU_SRC_BLOCK",
        help = "The block to begin the indexing from.",
        default_value_t = 0
    )]
    src: BlockNumber,

    #[arg(
        long = "dst-block",
        env = "CHAINTHRU_DST_BLOCK",
        help = "The block to end the indexing at.",
        default_value_t = BlockNumber::MAX
    )]
    dst: BlockNumber,
}

#[derive(Debug, clap::Args, Clone)]
#[group(multiple = false)]
pub(crate) struct BlockGroup {
    #[clap(flatten)]
    block: Block,

    #[arg(
        long = "from-latest",
        env = "CHAINTHRU_FROM_LATEST",
        help = "Toggler enabling|disabling the indexer to run from the latest block",
        action
    )]
    latest: bool,
}

#[derive(Debug, clap::Args, Clone)]
#[group(multiple = false)]
pub(crate) struct CriteriasGroup {
    /// file holding the criterias used to filter events
    #[arg(
        long,
        env = "CHAINTHRU_CRITERIAS_FILE",
        help = "file holding the criterias that'll be used to filter events"
    )]
    pub(crate) criterias_file: Option<String>,

    /// argument holding the criterias used to filter events
    #[arg(
        long,
        env = "CHAINTHRU_CRITERIAS_JSON",
        help = "argument holding the criterias that'll be used to filter events"
    )]
    pub(crate) criterias_json: Option<String>,
}

#[derive(Debug, clap::Args, Clone)]
#[group(skip)]
pub(crate) struct Events {
    #[clap(flatten)]
    pub(crate) criterias: CriteriasGroup,
}

#[derive(Debug, clap::Args, Clone)]
#[group(skip)]
pub(crate) struct IndexerSettings {
    #[arg(
        long = "indexer.enabled",
        env = "CHAINTHRU_INDEXER_ENABLED",
        help = "Toggler enabling|disabling the indexer",
        action
    )]
    pub(crate) indexer_enabled: bool,

    #[clap(flatten)]
    pub(crate) block: BlockGroup,

    #[clap(flatten)]
    pub(crate) events: Events,
}

#[derive(Debug, clap::Args, Clone)]
#[group(skip)]
pub(crate) struct ServerSettings {
    #[arg(
        long = "server.enabled",
        env = "CHAINTHRU_SERVER_ENABLED",
        help = "Toggler enabling|disabling the HTTP-API server",
        action
    )]
    pub(crate) server_enabled: bool,

    #[arg(
        long = "server.threads",
        env = "CHAINTHRU_SERVER_THREADS",
        help = "The number of threads to use for the API server",
        default_value_t = num_cpus::get(),
    )]
    pub(crate) server_threads: usize,

    #[arg(
        long = "server.host",
        env = "CHAINTHRU_SERVER_HOST",
        help = "The host to run the HTTP-API server on",
        default_value = "127.0.0.1"
    )]
    pub(crate) host: String,

    #[arg(
        long = "server.port",
        env = "CHAINTHRU_SERVER_PORT",
        help = "The port to run the HTTP-API server on",
        default_value_t = 6969,
        value_parser = clap::value_parser!(u16).range(1..),
    )]
    pub(crate) port: u16,
}

#[derive(Clone, Debug, Parser)]
#[command(about = "Index Ethereum into a Postgresql database & serve it through an HTTP-API.")]
pub(crate) struct Cmd {
    #[clap(flatten)]
    pub(crate) indexer: IndexerSettings,

    #[clap(flatten)]
    pub(crate) server: ServerSettings,

    #[arg(
        long = "only-migrations",
        env = "CHAINTHRU_DB_MIGRATIONS",
        help = "Run only the database migrations and exit.",
        action
    )]
    pub(crate) only_migrations: bool,

    #[arg(
        long,
        env = "CHAINTHRU_STORAGE_URL",
        help = "The database URL to connect to",
        default_value = "postgres://postgres:password@localhost:5432/chainthru"
    )]
    pub(crate) storage_url: Secret<String>,

    #[arg(
        long,
        env = "CHAINTHRU_NODE_URL",
        help = "The Ethereum node URL to connect to",
        default_value = "wss://eth.llamarpc.com"
    )]
    pub(crate) node_url: String,
}

impl From<Cmd> for server::Settings {
    fn from(settings: Cmd) -> Self {
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

// clippy complains that the functions aren't used, and that's not true
// therefore we're marking them as unused
impl Cmd {
    #[allow(unused)]
    pub(crate) fn indexer_enabled(&self) -> bool {
        self.indexer.indexer_enabled
    }

    #[allow(unused)]
    pub(crate) fn src_block(&self) -> BlockNumber {
        if self.indexer.block.latest {
            BlockNumber::MAX
        } else {
            self.indexer.block.block.src
        }
    }

    #[allow(unused)]
    pub(crate) fn dst_block(&self) -> BlockNumber {
        if self.indexer.block.latest {
            BlockNumber::MAX
        } else {
            self.indexer.block.block.dst
        }
    }

    #[allow(unused)]
    pub(crate) fn server_enabled(&self) -> bool {
        self.server.server_enabled
    }

    #[allow(unused)]
    pub(crate) fn server_threads(&self) -> usize {
        self.server.server_threads
    }

    #[allow(unused)]
    pub(crate) fn storage_url(&self) -> &str {
        self.storage_url.expose_secret()
    }

    #[allow(unused)]
    pub(crate) fn node_url(&self) -> &str {
        &self.node_url
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Args;

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
    fn test_indexer_settings_default_values() {
        let args = CommandParser::<IndexerSettings>::parse_from(["run"]).args;
        assert!(!args.indexer_enabled);
        assert!(!args.block.latest);
        assert_eq!(args.block.block.src, 0);
        assert_eq!(args.block.block.dst, BlockNumber::MAX);
        assert_eq!(args.events.criterias.criterias_file, None);
        assert_eq!(args.events.criterias.criterias_json, None);
    }

    #[test]
    #[serial]
    fn test_indexer_settings_env_values() {
        std::env::set_var("CHAINTHRU_INDEXER_ENABLED", "true");
        std::env::set_var("CHAINTHRU_SRC_BLOCK", "1");
        std::env::set_var("CHAINTHRU_DST_BLOCK", "2");
        std::env::set_var("CHAINTHRU_CRITERIAS_FILE", "tmp/criterias.rnd");

        let args = CommandParser::<IndexerSettings>::parse_from(["run"]).args;
        assert!(args.indexer_enabled);
        assert_eq!(args.block.block.src, 1);
        assert_eq!(args.block.block.dst, 2);
        assert_eq!(
            args.events.criterias.criterias_file,
            Some("tmp/criterias.rnd".into())
        );
        assert_eq!(args.events.criterias.criterias_json, None);

        std::env::remove_var("CHAINTHRU_INDEXER_ENABLED");
        std::env::remove_var("CHAINTHRU_SRC_BLOCK");
        std::env::remove_var("CHAINTHRU_DST_BLOCK");
        std::env::remove_var("CHAINTHRU_CRITERIAS_FILE");
    }

    #[test]
    #[serial]
    fn test_indexer_settings_args_precedence() {
        std::env::set_var("CHAINTHRU_SRC_BLOCK", "1");
        std::env::set_var("CHAINTHRU_DST_BLOCK", "2");
        std::env::set_var("CHAINTHRU_CRITERIAS_JSON", "[1,2,3]");

        let args = CommandParser::<IndexerSettings>::parse_from([
            "run",
            "--indexer.enabled",
            "--src-block",
            "3",
            "--dst-block",
            "4",
            "--criterias-json",
            "{\"a\": \"1\"},",
        ])
        .args;
        assert!(args.indexer_enabled);
        assert!(!args.block.latest);
        assert_eq!(args.block.block.src, 3);
        assert_eq!(args.block.block.dst, 4);
        assert_eq!(
            args.events.criterias.criterias_json,
            Some("{\"a\": \"1\"},".into())
        );
        assert_eq!(args.events.criterias.criterias_file, None);

        std::env::remove_var("CHAINTHRU_INDEXER_ENABLED");
        std::env::remove_var("CHAINTHRU_SRC_BLOCK");
        std::env::remove_var("CHAINTHRU_DST_BLOCK");
        std::env::remove_var("CHAINTHRU_CRITERIAS_JSON");
    }

    #[test]
    #[serial]
    fn test_run_subcmd_latest() {
        let args =
            CommandParser::<Cmd>::parse_from(["run", "--indexer.enabled", "--from-latest"]).args;

        assert_eq!(args.src_block(), BlockNumber::MAX);
        assert_eq!(args.dst_block(), BlockNumber::MAX);
    }

    #[test]
    #[serial]
    fn test_server_settings_default_values() {
        let args = CommandParser::<ServerSettings>::parse_from(["run"]).args;
        assert!(!args.server_enabled);
        assert_eq!(args.server_threads, num_cpus::get());
        assert_eq!(args.host, "127.0.0.1");
        assert_eq!(args.port, 6969);
    }

    #[test]
    #[serial]
    fn test_server_settings_env_values() {
        std::env::set_var("CHAINTHRU_SERVER_ENABLED", "true");
        std::env::set_var("CHAINTHRU_SERVER_THREADS", "1");
        std::env::set_var("CHAINTHRU_SERVER_HOST", "localhost");
        std::env::set_var("CHAINTHRU_SERVER_PORT", "1234");

        let args = CommandParser::<ServerSettings>::parse_from(["run"]).args;
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
            "run",
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
