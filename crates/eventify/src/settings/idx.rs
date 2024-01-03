use alloy_primitives::BlockNumber;
use clap::Args;

use eventify_primitives::Criterias;

#[derive(Args, Clone, Debug)]
#[group(skip)]
pub(crate) struct IdxSettings {
    #[arg(
        long = "indexer.enabled",
        env = "EVENTIFY_INDEXER_ENABLED",
        help = "Toggler enabling|disabling the indexer",
        action
    )]
    pub(crate) indexer_enabled: bool,

    #[arg(
        long = "skip-transactions",
        env = "EVENTIFY_SKIP_TRANSACTIONS",
        help = "Toggler enabling|disabling the indexer to skip transactions",
        action
    )]
    pub(crate) skip_transactions: bool,

    #[arg(
        long = "skip-blocks",
        env = "EVENTIFY_SKIP_BLOCKS",
        help = "Toggler enabling|disabling the indexer to skip blocks",
        action
    )]
    pub(crate) skip_blocks: bool,

    #[clap(flatten)]
    pub(crate) block: BlockGroup,

    #[clap(flatten)]
    pub(crate) events: Events,
}

#[derive(Args, Clone, Debug)]
pub(crate) struct Block {
    #[arg(
        long = "src-block",
        env = "EVENTIFY_SRC_BLOCK",
        help = "The block to begin the indexing from.",
        default_value_t = 0
    )]
    pub(crate) src: BlockNumber,

    #[arg(
        long = "dst-block",
        env = "EVENTIFY_DST_BLOCK",
        help = "The block to end the indexing at.",
        default_value_t = BlockNumber::MAX
    )]
    pub(crate) dst: BlockNumber,
}

#[derive(Args, Clone, Debug)]
#[group(multiple = false)]
pub(crate) struct BlockGroup {
    #[clap(flatten)]
    pub(crate) block: Block,

    #[arg(
        long = "from-latest",
        env = "EVENTIFY_FROM_LATEST",
        help = "Toggler enabling|disabling the indexer to run from the latest block",
        action
    )]
    pub(crate) latest: bool,
}

#[derive(Args, Clone, Debug)]
#[group(multiple = false)]
pub(crate) struct CriteriasGroup {
    #[arg(
        long,
        env = "EVENTIFY_CRITERIAS_FILE",
        help = "file holding the criterias that'll be used to filter events",
        default_value = None,
    )]
    pub(crate) criterias_file: Option<String>,

    #[arg(
        long,
        env = "EVENTIFY_CRITERIAS_JSON",
        help = "Argument holding the criterias that'll be used to filter events",
        default_value = None,
        value_parser = clap::value_parser!(Criterias)
    )]
    pub(crate) criterias_json: Option<Criterias>,
}

#[derive(Args, Clone, Debug)]
#[group(skip)]
pub(crate) struct Events {
    #[clap(flatten)]
    pub(crate) criterias: CriteriasGroup,
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;
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
    fn test_indexer_settings_default_values() {
        let args = CommandParser::<IdxSettings>::parse_from(["run"]).args;
        assert!(!args.indexer_enabled);
        assert!(!args.skip_transactions);
        assert!(!args.skip_blocks);
        assert!(!args.block.latest);
        assert_eq!(args.block.block.src, 0);
        assert_eq!(args.block.block.dst, BlockNumber::MAX);
        assert_eq!(args.events.criterias.criterias_file, None);
        assert_eq!(args.events.criterias.criterias_json, None);
    }

    #[test]
    #[serial]
    fn test_indexer_settings_env_values() {
        set_var("EVENTIFY_INDEXER_ENABLED", "true");
        set_var("EVENTIFY_SKIP_TRANSACTIONS", "true");
        set_var("EVENTIFY_SKIP_BLOCKS", "true");
        set_var("EVENTIFY_SRC_BLOCK", "1");
        set_var("EVENTIFY_DST_BLOCK", "2");
        set_var("EVENTIFY_CRITERIAS_FILE", "tmp/criterias.rnd");

        let args = CommandParser::<IdxSettings>::parse_from(["run"]).args;
        assert!(args.indexer_enabled);
        assert!(args.skip_transactions);
        assert!(args.skip_blocks);
        assert_eq!(args.block.block.src, 1);
        assert_eq!(args.block.block.dst, 2);
        assert_eq!(
            args.events.criterias.criterias_file,
            Some("tmp/criterias.rnd".into())
        );
        assert_eq!(args.events.criterias.criterias_json, None);

        remove_var("EVENTIFY_INDEXER_ENABLED");
        remove_var("EVENTIFY_SKIP_TRANSACTIONS");
        remove_var("EVENTIFY_SKIP_BLOCKS");
        remove_var("EVENTIFY_SRC_BLOCK");
        remove_var("EVENTIFY_DST_BLOCK");
        remove_var("EVENTIFY_CRITERIAS_FILE");
    }

    #[test]
    #[serial]
    fn test_indexer_settings_args_precedence() {
        set_var("EVENTIFY_SRC_BLOCK", "1");
        set_var("EVENTIFY_DST_BLOCK", "2");
        set_var("EVENTIFY_CRITERIAS_JSON", "[{\"name\":\"UniswapV3Factory\",\"events\":[\"PoolCreated(address,address,uint24,int24,address)\"],\"addresses\":[\"0x1F98431c8aD98523631AE4a59f267346ea31F984\"]},{\"name\":\"ERC20\",\"events\":[\"Transfer(address,address,uint256)\",\"Approve(address,address,uint256)\"],\"addresses\":[\"0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2\",\"0x6B175474E89094C44Da98b954EedeAC495271d0F\"]}]");

        let args = CommandParser::<IdxSettings>::parse_from([
            "run",
            "--indexer.enabled",
            "--src-block",
            "3",
            "--dst-block",
            "4",
            "--criterias-json",
            "[{\"name\":\"UniswapV3Swap\",\"events\":[\"Swap(address,address,int256,int256,uint160,uint128,int24)\"],\"addresses\":[\"0x1F98431c8aD98523631AE4a59f267346ea31F984\"]}]",
        ])
        .args;
        assert!(args.indexer_enabled);
        assert!(!args.block.latest);
        assert_eq!(args.block.block.src, 3);
        assert_eq!(args.block.block.dst, 4);
        assert_eq!(
            args.events.criterias.criterias_json,
            Some("[{\"name\":\"UniswapV3Swap\",\"events\":[\"Swap(address,address,int256,int256,uint160,uint128,int24)\"],\"addresses\":[\"0x1F98431c8aD98523631AE4a59f267346ea31F984\"]}]".into())
        );
        assert_eq!(args.events.criterias.criterias_file, None);

        remove_var("EVENTIFY_SRC_BLOCK");
        remove_var("EVENTIFY_DST_BLOCK");
        remove_var("EVENTIFY_CRITERIAS_JSON");
    }
}
