pub mod eth;

use std::{fmt::Display, num::ParseIntError};

use crate::Error;
use alloy_primitives::{BlockNumber, B256};
use eventify_primitives::{Criteria, EthBlock, EthLog, EthTransaction};
use eyre::Result;
use jsonrpsee::core::client::Subscription;

#[trait_variant::make(NodeProvider: Send)]
pub trait LocalNodeProvider: 'static + Clone + Sync {
    async fn connect(host: String) -> Result<Self>;

    async fn get_block_number(&self) -> Result<BlockNumber>;
    async fn get_block(&self, block: BlockNumber) -> Result<EthBlock>;
    async fn get_transactions(&self, n: BlockNumber) -> Result<Vec<EthTransaction>>;
    async fn get_logs(&self, filter: &Criteria) -> Result<Vec<EthLog>>;

    async fn stream_blocks(&self) -> Result<Subscription<EthBlock>>;
    async fn stream_transactions(&self) -> Result<Subscription<B256>>;
    async fn stream_logs(&self) -> Result<Subscription<EthLog>>;
}

#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum NodeProviderError {
    #[error("Failed to connect to node: {0}")]
    ConnectionFailed(#[from] jsonrpsee::core::ClientError),

    #[error("Failed to get the latest block number")]
    GetLatestBlockFailed,

    #[error("failed to get block {0}")]
    GetBlockFailed(u64),

    #[error("failed to get transactions from block {0}")]
    GetTransactionsFailed(u64),

    #[error("Failed to get logs for criteria {0}")]
    Logs(String),

    #[error("Failed to get block from sub {0}, with params {1}")]
    BlockSubscriptionFailed(String, String),

    #[error("Failed to get transaction from sub {0}, with params {1}")]
    TransactionSubscriptionFailed(String, String),

    #[error("Failed to get log from sub {0}, with params {1}")]
    LogSubscriptionFailed(String, String),

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
