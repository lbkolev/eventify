use alloy_primitives::BlockNumber;

use eventify_primitives::{Block, Criteria, Log, Transaction};

#[async_trait::async_trait]
pub trait NodeProvider<E>: Send + Sync + Clone + 'static
where
    E: std::error::Error + Send + Sync,
{
    async fn new(url: &str) -> Result<Self, E>;
    async fn connect(url: &str) -> Result<Self, E>;

    async fn get_block_number(&self) -> Result<u64, E>;
    async fn get_block(&self, block: BlockNumber) -> Result<Block, E>;
    async fn get_transactions(&self, block: BlockNumber) -> Result<Vec<Transaction>, E>;
    async fn get_logs(&self, criterias: &Criteria) -> Result<Vec<Log>, E>;
}
