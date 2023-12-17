use alloy_primitives::BlockNumber;
use ethers_core::{
    k256::elliptic_curve::rand_core::block,
    types::{Block, Transaction},
};
use eventify_primitives::Criteria;

#[async_trait::async_trait]
pub trait NodeProvider: Send + Sync + Clone + 'static {
    type Error: std::error::Error + Send + Sync + 'static;

    async fn get_block_number(&self) -> Result<u64, Self::Error>;

    async fn get_block(&self, block: BlockNumber) -> Result<Block<()>, Self::Error>;
    async fn get_transactions(&self, block: BlockNumber) -> Result<Vec<Transaction>, Self::Error>;
    async fn get_logs(&self, criteria: Criteria) -> Result<(), Self::Error>;

    async fn stream_blocks(&self) -> Result<(), Self::Error>;
    async fn stream_transactions(&self) -> Result<(), Self::Error>;
    async fn stream_logs(&self, criteria: Criteria) -> Result<(), Self::Error>;
}
