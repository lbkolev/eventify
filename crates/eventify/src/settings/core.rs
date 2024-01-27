use alloy_primitives::BlockNumber;
use clap::Args;

use eventify_primitives::Criteria;

#[derive(Args, Clone, Debug)]
#[group(skip)]
pub(crate) struct CoreSettings {
    #[arg(
        long = "skip-transactions",
        env = "EVENTIFY_SKIP_TRANSACTIONS",
        help = "Toggler enabling|disabling the eventifier to skip transactions",
        action
    )]
    pub(crate) skip_transactions: bool,

    #[arg(
        long = "skip-blocks",
        env = "EVENTIFY_SKIP_BLOCKS",
        help = "Toggler enabling|disabling the eventifier to skip blocks",
        action
    )]
    pub(crate) skip_blocks: bool,

    #[arg(
        long = "skip-logs",
        env = "EVENTIFY_SKIP_LOGS",
        help = "Toggler enabling|disabling the eventifier to skip logs",
        action
    )]
    pub(crate) skip_logs: bool,

    #[clap(flatten)]
    pub(crate) block: BlockGroup,

    #[clap(flatten)]
    pub(crate) events: Events,
}

#[derive(Args, Clone, Debug, Eq, PartialEq)]
pub(crate) struct BlockRange {
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

    #[arg(
        long = "step",
        env = "EVENTIFY_STEP",
        help = "The step to use when indexing blocks.",
        default_value_t = 1
    )]
    pub(crate) step: BlockNumber,
}

impl From<BlockRange> for eventify_configs::configs::BlockRange {
    fn from(block: BlockRange) -> Self {
        Self {
            src: block.src,
            dst: block.dst,
            step: block.step,
        }
    }
}

#[derive(Args, Clone, Debug)]
#[group(multiple = false)]
pub(crate) struct BlockGroup {
    #[clap(flatten)]
    pub(crate) block: Option<BlockRange>,

    #[arg(
        long = "from-latest",
        env = "EVENTIFY_FROM_LATEST",
        help = "Toggler enabling|disabling the indexer to run from the latest block",
        default_value = "true",
        action
    )]
    pub(crate) from_latest: bool,
}

#[derive(Args, Clone, Debug)]
#[group(multiple = false)]
pub(crate) struct CriteriaGroup {
    #[arg(
        long,
        env = "EVENTIFY_CRITERIA_FILE",
        help = "file holding the criteria that'll be used to filter events",
        default_value = None,
    )]
    pub(crate) criteria_file: Option<String>,

    #[arg(
        long,
        env = "EVENTIFY_CRITERIA_JSON",
        help = "Argument holding the criteria that'll be used to filter events",
        default_value = None,
        value_parser = clap::value_parser!(Criteria)
    )]
    pub(crate) criteria_json: Option<Criteria>,
}

#[derive(Args, Clone, Debug)]
#[group(skip)]
pub(crate) struct Events {
    #[clap(flatten)]
    pub(crate) criteria: CriteriaGroup,
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
    fn test_run_settings_default_values() {
        let args = CommandParser::<CoreSettings>::parse_from(["run"]).args;
        assert!(!args.skip_transactions);
        assert!(!args.skip_blocks);
        assert!(!args.skip_logs);
        assert!(args.block.block.is_none());
        assert_eq!(args.events.criteria.criteria_file, None);
        assert_eq!(args.events.criteria.criteria_json, None);
    }

    #[test]
    #[serial]
    fn test_run_settings_env_values() {
        set_var("EVENTIFY_SKIP_TRANSACTIONS", "true");
        set_var("EVENTIFY_SKIP_BLOCKS", "true");
        set_var("EVENTIFY_SRC_BLOCK", "1");
        set_var("EVENTIFY_DST_BLOCK", "2");
        set_var("EVENTIFY_CRITERIA_FILE", "tmp/criteria.rnd");

        let args = CommandParser::<CoreSettings>::parse_from(["run"]).args;
        assert!(args.skip_transactions);
        assert!(args.skip_blocks);
        assert_eq!(
            args.block.block.unwrap(),
            BlockRange {
                src: 1,
                dst: 2,
                step: 1
            }
        );
        assert_eq!(
            args.events.criteria.criteria_file,
            Some("tmp/criteria.rnd".into())
        );
        assert_eq!(args.events.criteria.criteria_json, None);

        remove_var("EVENTIFY_SKIP_TRANSACTIONS");
        remove_var("EVENTIFY_SKIP_BLOCKS");
        remove_var("EVENTIFY_SRC_BLOCK");
        remove_var("EVENTIFY_DST_BLOCK");
        remove_var("EVENTIFY_CRITERIA_FILE");
    }

    #[test]
    #[serial]
    fn test_indexer_settings_args_precedence() {
        set_var("EVENTIFY_SRC_BLOCK", "1");
        set_var("EVENTIFY_DST_BLOCK", "2");
        set_var("EVENTIFY_CRITERIA_JSON", "[{\"name\":\"UniswapV3Factory\",\"events\":[\"PoolCreated(address,address,uint24,int24,address)\"],\"addresses\":[\"0x1F98431c8aD98523631AE4a59f267346ea31F984\"]},{\"name\":\"ERC20\",\"events\":[\"Transfer(address,address,uint256)\",\"Approve(address,address,uint256)\"],\"addresses\":[\"0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2\",\"0x6B175474E89094C44Da98b954EedeAC495271d0F\"]}]");

        let args = CommandParser::<CoreSettings>::parse_from([
            "run",
            "--src-block",
            "3",
            "--dst-block",
            "4",
            "--criteria-json",
            "[{\"name\":\"UniswapV3Swap\",\"events\":[\"Swap(address,address,int256,int256,uint160,uint128,int24)\"],\"addresses\":[\"0x1F98431c8aD98523631AE4a59f267346ea31F984\"]}]",
        ])
        .args;
        assert_eq!(
            args.block.block.unwrap(),
            BlockRange {
                src: 3,
                dst: 4,
                step: 1
            }
        );
        assert_eq!(
            args.events.criteria.criteria_json,
            Some("[{\"name\":\"UniswapV3Swap\",\"events\":[\"Swap(address,address,int256,int256,uint160,uint128,int24)\"],\"addresses\":[\"0x1F98431c8aD98523631AE4a59f267346ea31F984\"]}]".into())
        );
        assert_eq!(args.events.criteria.criteria_file, None);

        remove_var("EVENTIFY_SRC_BLOCK");
        remove_var("EVENTIFY_DST_BLOCK");
        remove_var("EVENTIFY_CRITERIA_JSON");
    }
}
