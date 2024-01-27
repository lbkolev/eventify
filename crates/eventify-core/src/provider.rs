pub mod eth;

use std::{fmt::Display, num::ParseIntError};

use crate::Error;
use alloy_primitives::{BlockNumber, B256};
use eventify_primitives::{Criteria, EthBlock, EthLog, EthTransaction};
use eyre::Result;
use jsonrpsee::core::client::Subscription;

#[trait_variant::make(NodeProvider: Send)]
pub trait LocalNodeProvider: 'static + Clone + Sync {
    async fn get_block_number(&self) -> Result<BlockNumber>;

    // block with tx hashes
    async fn get_block(&self, block: BlockNumber) -> Result<EthBlock<B256>>;
    async fn get_transactions(&self, n: BlockNumber) -> Result<Vec<EthTransaction>>;
    async fn get_logs(&self, criteria: &Criteria) -> Result<Vec<EthLog>>;

    async fn stream_blocks(&self) -> Result<Subscription<EthBlock<B256>>>;
    async fn stream_logs(&self) -> Result<Subscription<EthLog>>;
}

#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum NodeProviderError {
    #[error("failed to connect to node: {0}")]
    ConnectionFailed(#[from] jsonrpsee::core::ClientError),

    #[error("failed to get the latest block number. {err}")]
    GetLatestBlockFailed { err: String },

    #[error("failed to get block #{n}. {err}")]
    GetBlockFailed { n: u64, err: String },

    #[error("failed to get transactions from block #{n}. {err}")]
    GetTransactionsFailed { n: u64, err: String },

    #[error("Failed to get logs. {err}")]
    GetLogsFailed { err: String },

    #[error("failed to get block from sub {sub}, with params {params}. {err}")]
    BlockSubscriptionFailed {
        sub: String,
        params: String,
        err: String,
    },

    #[error("failed to get log from sub {sub}, with params {params}. {err}")]
    LogSubscriptionFailed {
        sub: String,
        params: String,
        err: String,
    },

    #[error(transparent)]
    ParseInt(#[from] ParseIntError),
}

// Supported chains
#[derive(Clone, Copy, Debug, Default)]
pub enum NodeKind {
    #[default]
    Ethereum,
}

impl Display for NodeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeKind::Ethereum => write!(f, "eth"),
        }
    }
}

impl std::str::FromStr for NodeKind {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "ethereum" | "eth" => Ok(NodeKind::Ethereum),
            _ => Err(Error::InvalidNodeKind(s.to_string())),
        }
    }
}
