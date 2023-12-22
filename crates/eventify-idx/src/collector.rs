use alloy_primitives::BlockNumber;

use crate::{
    types::{collect::Collect, provider::NodeProvider},
    Result,
};
use eventify_primitives::{Block, Criteria, Storage};

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
}

#[async_trait::async_trait]
impl<N, S> Collect<Criteria, crate::Error> for Collector<N, S>
where
    N: NodeProvider<crate::Error>,
    S: Storage,
{
    async fn process_block(&self, block: BlockNumber) -> Result<()> {
        let block = self
            .node
            .get_block(block)
            .await
            .map_err(|e| crate::Error::FetchBlock(format!("Failed to fetch block: {}", e)))?;
        self.storage.store_block(&block).await?;

        Ok(())
    }

    async fn process_blocks(&self, from: BlockNumber, to: BlockNumber) -> Result<()> {
        for block in from..=to {
            self.process_block(block).await?;
        }

        Ok(())
    }

    async fn process_transactions(&self, block: BlockNumber) -> Result<()> {
        let transactions = self.node.get_transactions(block).await.map_err(|e| {
            crate::Error::FetchBlock(format!("Failed to fetch transactions: {}", e))
        })?;

        for tx in transactions {
            self.storage.store_transaction(&tx).await?;
        }

        Ok(())
    }

    async fn process_transactions_from_range(
        &self,
        from: BlockNumber,
        to: BlockNumber,
    ) -> Result<()> {
        for block in from..=to {
            self.process_transactions(block).await?;
        }

        Ok(())
    }

    async fn process_logs(&self, c: Criteria) -> Result<()> {
        let logs = self.node.get_logs(c).await?;

        for log in logs {
            self.storage.store_log(&log).await?;
        }

        Ok(())
    }
}
