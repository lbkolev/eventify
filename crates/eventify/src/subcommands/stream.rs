use std::str::FromStr;

use clap::Parser;

use eventify_idx::provider::NodeKind;

// TODO: implement the stream subcmd
#[derive(Clone, Debug, Parser)]
#[command(about = "[NOT YET IMPLEMENTED] Subscribe & stream directly from the tip of the chain")]
pub(crate) struct Cmd {
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
