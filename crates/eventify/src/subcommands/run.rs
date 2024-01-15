use std::str::FromStr;

use alloy_primitives::BlockNumber;
use clap::{self, Parser};
use secrecy::{ExposeSecret, Secret};

use crate::settings::{IdxSettings, ServerSettings};
use eventify_idx::clients::NodeKind;
use eventify_primitives::{
    configs::{ApplicationConfig, DatabaseConfig, ServerConfig},
    Criterias,
};

#[derive(Clone, Debug, Parser)]
#[command(about = "L1/L2 Indexer & Event listener.")]
pub(crate) struct Cmd {
    #[clap(flatten)]
    pub(crate) indexer: IdxSettings,

    #[clap(flatten)]
    pub(crate) server: ServerSettings,

    #[arg(
        long = "only-migrations",
        env = "EVENTIFY_DB_MIGRATIONS",
        help = "Run only the database migrations and exit immediately after.",
        action
    )]
    pub(crate) only_migrations: bool,

    #[arg(
        long,
        env = "DATABASE_URL",
        help = "The database URL to connect to",
        default_value = "postgres://postgres:password@localhost:5432/eventify"
    )]
    pub(crate) database_url: Secret<String>,

    #[arg(
        long,
        env = "EVENTIFY_NODE_URL",
        help = "The node URL to connect to",
        default_value = "wss://eth.llamarpc.com"
    )]
    pub(crate) node_url: String,

    #[arg(
        long,
        env = "EVENTIFY_NODE",
        help = "The type of chain(node) to index",
        default_value_t = NodeKind::Ethereum,
        value_parser = NodeKind::from_str,
    )]
    pub(crate) node: NodeKind,
}

impl From<Cmd> for ServerConfig {
    fn from(settings: Cmd) -> Self {
        Self {
            database: DatabaseConfig::from(settings.database_url()),
            application: ApplicationConfig {
                host: settings.server.host,
                port: settings.server.port,
                worker_threads: settings.server.server_threads,
            },
        }
    }
}

impl Cmd {
    #[allow(unused)]
    pub(crate) fn indexer_enabled(&self) -> bool {
        self.indexer.indexer_enabled
    }

    pub(crate) fn skip_transactions(&self) -> bool {
        self.indexer.skip_transactions
    }

    pub(crate) fn skip_blocks(&self) -> bool {
        self.indexer.skip_blocks
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

    pub(crate) fn criterias_file(&self) -> Option<String> {
        self.indexer.events.criterias.criterias_file.clone()
    }

    pub(crate) fn criterias_json(&self) -> Option<Criterias> {
        self.indexer.events.criterias.criterias_json.clone()
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
    pub(crate) fn database_url(&self) -> &str {
        self.database_url.expose_secret()
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
    fn test_run_subcmd_latest() {
        let args =
            CommandParser::<Cmd>::parse_from(["run", "--indexer.enabled", "--from-latest"]).args;

        assert_eq!(args.src_block(), BlockNumber::MAX);
        assert_eq!(args.dst_block(), BlockNumber::MAX);
    }
}
