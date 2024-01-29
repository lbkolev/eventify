use futures::Future;
use std::{num::ParseIntError};


use alloy_primitives::{BlockNumber, B256};
use eventify_primitives::{Criteria, EthBlock, EthLog, EthTransaction};
use eyre::Result;
use jsonrpsee::core::client::Subscription;

pub trait Node: 'static + Clone + Sync + Send {
    fn get_block_number(&self) -> impl Future<Output = Result<BlockNumber>> + Send;

    // block with tx hashes
    fn get_block(&self, block: BlockNumber) -> impl Future<Output = Result<EthBlock<B256>>> + Send;
    fn get_transactions(
        &self,
        n: BlockNumber,
    ) -> impl Future<Output = Result<Vec<EthTransaction>>> + Send;
    fn get_logs(&self, criteria: &Criteria) -> impl Future<Output = Result<Vec<EthLog>>> + Send;

    fn stream_blocks(&self) -> impl Future<Output = Result<Subscription<EthBlock<B256>>>> + Send;
    fn stream_logs(&self) -> impl Future<Output = Result<Subscription<EthLog>>> + Send;
}

#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum NodeError {
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
