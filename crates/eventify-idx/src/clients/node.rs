pub mod eth;

use alloy_primitives::BlockNumber;

use crate::error::NodeClientError;
use eventify_primitives::{Block, Criteria, Log, Transaction};

#[async_trait::async_trait]
pub trait NodeClient: Send + Sync + Clone + 'static {
    async fn get_block_number(&self) -> Result<u64, NodeClientError>;
    async fn get_block(&self, block: BlockNumber) -> Result<Block, NodeClientError>;
    async fn get_transactions(
        &self,
        block: BlockNumber,
    ) -> Result<Vec<Transaction>, NodeClientError>;
    async fn get_logs(&self, criterias: &Criteria) -> Result<Vec<Log>, NodeClientError>;
}

#[async_trait::async_trait]
pub trait Auth {
    async fn connect(url: &str) -> Self;
}
