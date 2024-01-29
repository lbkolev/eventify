use std::{collections::HashMap, hash::Hash};

use alloy_primitives::BlockNumber;

use eventify_primitives::{Criteria, NetworkKind};

#[derive(Clone, Debug, Default)]
pub struct ManagerConfig {
    pub skip_blocks: bool,
    pub skip_transactions: bool,
    pub skip_logs: bool,
    pub criteria: Option<Criteria>,

    // either walking through published blocks or following the tip of the chain
    pub range: Option<BlockRange>,
}

impl ManagerConfig {
    pub fn new(
        skip_blocks: bool,
        skip_transactions: bool,
        skip_logs: bool,
        criteria: Option<Criteria>,
        range: Option<BlockRange>,
    ) -> Self {
        Self {
            skip_blocks,
            skip_transactions,
            skip_logs,
            criteria,
            range,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BlockRange {
    pub src: BlockNumber,
    pub dst: BlockNumber,
    pub step: BlockNumber,
}

#[derive(Clone, Debug, Default)]
pub struct CollectorConfig {
    pub network: NetworkKind,
}

impl CollectorConfig {
    pub fn new(network: NetworkKind) -> Self {
        Self { network }
    }
}
