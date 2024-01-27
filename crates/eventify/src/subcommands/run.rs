use std::str::FromStr;

use alloy_primitives::BlockNumber;
use clap::Parser;
use secrecy::{ExposeSecret, Secret};

use crate::settings::{core::BlockRange, CoreSettings, ServerSettings};
use eventify_configs::configs::{ApplicationConfig, DatabaseConfig, ServerConfig};
use eventify_core::provider::NodeKind;
use eventify_primitives::Criteria;

#[derive(Clone, Debug, Parser)]
#[command(about = "Idx from range or stream directly from the tip of the chain")]
pub(crate) struct Cmd {
    #[clap(flatten)]
    pub(crate) core: CoreSettings,

    #[clap(flatten)]
    pub(crate) server: ServerSettings,

    #[arg(
        long,
        env = "DATABASE_URL",
        help = "The database URL to connect to",
        default_value = "postgres://postgres:password@localhost:5432/eventify"
    )]
    pub(crate) database_url: Secret<String>,

    #[arg(
        long,
        env = "EVENTIFY_NODE",
        help = "The type of chain(node) to index",
        default_value_t = NodeKind::Ethereum,
        value_parser = NodeKind::from_str,
    )]
    pub(crate) node: NodeKind,

    #[arg(
        long,
        env = "EVENTIFY_NODE_URL",
        help = "The node URL to connect to",
        default_value = "wss://eth.llamarpc.com"
    )]
    pub(crate) node_url: String,
}

impl Cmd {
    pub(crate) fn skip_transactions(&self) -> bool {
        self.core.skip_transactions
    }

    pub(crate) fn skip_blocks(&self) -> bool {
        self.core.skip_blocks
    }

    pub(crate) fn skip_logs(&self) -> bool {
        self.core.skip_logs
    }

    pub(crate) fn src_block(&self) -> Option<BlockNumber> {
        self.core.block.block.as_ref().map(|block| block.src)
    }

    pub(crate) fn dst_block(&self) -> Option<BlockNumber> {
        self.core.block.block.as_ref().map(|block| block.dst)
    }

    pub(crate) fn block_step(&self) -> Option<BlockNumber> {
        self.core.block.block.as_ref().map(|block| block.step)
    }

    pub(crate) fn from_latest(&self) -> bool {
        self.core.block.from_latest
    }

    pub(crate) fn block_range(&self) -> Option<BlockRange> {
        self.core.block.block.clone()
    }

    pub(crate) fn criteria_file(&self) -> Option<String> {
        self.core.events.criteria.criteria_file.clone()
    }

    pub(crate) fn criteria_json(&self) -> Option<Criteria> {
        self.core.events.criteria.criteria_json.clone()
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
        let args = CommandParser::<Cmd>::parse_from(["run", "--from-latest"]).args;

        assert_eq!(args.src_block(), None);
        assert_eq!(args.dst_block(), None);
    }
}
