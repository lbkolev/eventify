use std::time::Instant;

use alloy_primitives::BlockNumber;
use tracing::info;

use crate::{
    types::{Collect, NodeProvider, StorageProvider},
    Result,
};
use eventify_primitives::Criteria;

#[derive(Debug, Clone)]
pub struct Collector<N, S>
where
    N: NodeProvider<crate::Error>,
    S: StorageProvider,
{
    node: N,
    storage: S,
}

impl<N, S> Collector<N, S>
where
    N: NodeProvider<crate::Error>,
    S: StorageProvider,
{
    pub fn new(node: N, storage: S) -> Self {
        Self { node, storage }
    }

    // todo
    //    pub async fn new_with_url(
    //        node_type: Chain,
    //        node_url: &str,
    //        storage_type: Database,
    //        storage_url: &str,
    //    ) -> Result<Self> {
    //        let node: Box<dyn NodeProvider<crate::Error>> = match node_type {
    //            Chain::Ethereum => match Url::parse(node_url)?.scheme() {
    //                "http" => Box::new(EthHttp::new(node_url).await?),
    //                "ws" => Box::new(EthWs::new(node_url).await?),
    //                "ipc" => Box::new(EthIpc::new(node_url).await?),
    //                _ => panic!("Invalid node url"),
    //            },
    //        };
    //
    //        let storage = match storage_type {
    //            Database::Postgres => Postgres::new(storage_url),
    //        };
    //
    //        Ok(Self::new(node, storage))
    //    }
    //
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
    S: StorageProvider,
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
        info!(target: "eventify::idx", from_block=?from, to_block=?to, "Processing blocks");
        let now = Instant::now();

        for block in from..=to {
            self.process_block(block).await?;

            if block % 30 == 0 {
                info!(target: "eventify::idx", processed=?true, latest=?block, elapsed=?now.elapsed(), "Processed {} blocks {}..{} |", block - from, from, to);
            }
        }

        Ok(())
    }

    async fn process_transactions(&self, block: BlockNumber) -> Result<()> {
        let transactions = self.node.get_transactions(block).await.map_err(|e| {
            crate::Error::FetchBlock(format!("Failed to fetch transactions: {}", e))
        })?;
        let count = transactions.len();
        let now = Instant::now();

        for tx in transactions {
            self.storage.store_transaction(&tx).await?;
        }
        info!(target: "eventify::idx", processed=?true, tx_count=?count, block=?block, elapsed=?now.elapsed());

        Ok(())
    }

    async fn process_transactions_from_range(
        &self,
        from: BlockNumber,
        to: BlockNumber,
    ) -> Result<()> {
        info!(target: "eventify::idx", "Processing transactions from blocks {}..{}", from, to);

        for block in from..=to {
            self.process_transactions(block).await?;
        }

        Ok(())
    }

    async fn process_logs(&self, c: Criteria) -> Result<()> {
        let logs = self.node.get_logs(&c).await?;
        let mut count = 0;

        for log in logs {
            self.storage.store_log(&log).await?;
            count += 1;

            if count % 100 == 0 {
                info!(target: "eventify::idx", processed=?true, log_count=?count, criteria_name=?c.name, latest_log=?log);
            }
        }

        Ok(())
    }
}
