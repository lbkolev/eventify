use alloy_primitives::BlockNumber;

use crate::{types::provider::NodeProvider, Result};
use eventify_primitives::{Block, Criterias, Log, Storage, Transaction};

#[derive(Debug, Clone)]
pub struct Collector<N, S>
where
    N: NodeProvider<crate::Error>,
    S: Storage,
{
    pub node: N,
    pub storage: S,
}

impl<N, S> Collector<N, S>
where
    N: NodeProvider<crate::Error>,
    S: Storage,
{
    pub fn new(node: N, storage: S) -> Self {
        Self { node, storage }
    }

    pub async fn get_latest_block(&self) -> Result<BlockNumber> {
        self.node
            .get_block_number()
            .await
            .map_err(|e| crate::Error::FetchBlock(format!("Failed to fetch latest block: {}", e)))
    }

    pub async fn fetch_block(&self, block: BlockNumber) -> Result<()> {
        let block = self
            .node
            .get_block(block)
            .await
            .map_err(|e| crate::Error::FetchBlock(format!("Failed to fetch block: {}", e)))?;
        self.storage.store_block(&Block::from(block)).await?;

        Ok(())
    }

    pub async fn fetch_blocks(&self, from: BlockNumber, to: BlockNumber) -> Result<()> {
        for block in from..=to {
            self.fetch_block(block).await?;
        }

        Ok(())
    }

    pub async fn fetch_blocks_from(&self, from: BlockNumber) -> Result<()> {
        let to = self.node.get_block_number().await?;
        self.fetch_blocks(from, to).await?;

        Ok(())
    }

    pub async fn fetch_transactions(&self, block: BlockNumber) -> Result<()> {
        let transactions = self.node.get_transactions(block).await.map_err(|e| {
            crate::Error::FetchBlock(format!("Failed to fetch transactions: {}", e))
        })?;

        for tx in transactions {
            self.storage
                .store_transaction(&Transaction::from(tx))
                .await?;
        }

        Ok(())
    }

    pub async fn fetch_transactions_from(&self, from: BlockNumber) -> Result<()> {
        let to = self.node.get_block_number().await?;
        for block in from..=to {
            self.fetch_transactions(block).await?;
        }

        Ok(())
    }

    pub async fn fetch_logs(&self, criterias: Criterias, block: BlockNumber) -> Result<()> {
        let log = self.node.get_logs(criterias, block).await?;

        for log in log {
            self.storage.store_log(&Log::from(log)).await?;
        }

        Ok(())
    }
}
