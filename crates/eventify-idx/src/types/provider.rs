use alloy_primitives::BlockNumber;
use ethers_core::types::{Block, Log, Transaction};

use eventify_primitives::Criterias;

#[async_trait::async_trait]
pub trait NodeProvider<E>: Send + Sync + Clone + 'static
where
    E: std::error::Error + Send + Sync,
{
    async fn new(url: &str) -> Result<Self, E>;
    async fn connect(url: &str) -> Result<Self, E>;

    async fn get_block_number(&self) -> Result<u64, E>;
    async fn get_block(&self, block: BlockNumber) -> Result<Block<Transaction>, E>;
    async fn get_transactions(&self, block: BlockNumber) -> Result<Vec<Transaction>, E>;
    async fn get_logs(&self, criteria: Criterias, block: BlockNumber) -> Result<Vec<Log>, E>;

    async fn stream_blocks(&self) -> Result<(), E>;
    async fn stream_transactions(&self) -> Result<(), E>;
    async fn stream_logs(&self, criteria: Criterias) -> Result<(), E>;
}
