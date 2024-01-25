use std::time::Instant;

use alloy_primitives::BlockNumber;
use tracing::{info, trace};

use crate::{provider::NodeProvider, Collect, StorageClient};
use eventify_primitives::Criteria;

#[derive(Debug, Clone)]
pub struct Collector<N, S>
where
    N: NodeProvider,
    S: StorageClient,
{
    node: N,
    storage: S,
}

impl<N, S> Collector<N, S>
where
    N: NodeProvider,
    S: StorageClient,
{
    pub fn new(node: N, storage: S) -> Self {
        Self { node, storage }
    }

    pub async fn get_latest_block(&self) -> crate::Result<BlockNumber> {
        self.node.get_block_number().await.map_err(Into::into)
    }
}

impl<N, S> Collect<crate::Error> for Collector<N, S>
where
    N: NodeProvider,
    S: StorageClient,
{
    async fn process_block(&self, block: BlockNumber) -> crate::Result<()> {
        let block = self.node.get_block(block).await?;
        self.storage.store_block(&block).await?;

        Ok(())
    }

    async fn process_blocks(&self, from: BlockNumber, to: BlockNumber) -> crate::Result<()> {
        info!(target: "eventify::idx", from_block=?from, to_block=?to, "Processing blocks");
        let now = Instant::now();

        for block in from..=to {
            self.process_block(block).await?;

            if block % 30 == 0 {
                info!(target: "eventify::idx::block", processed=?true, block_count=?block - from, latest=?block, elapsed=?now.elapsed());
            }
        }

        Ok(())
    }

    async fn process_transactions(&self, block: BlockNumber) -> crate::Result<()> {
        let now = Instant::now();
        let transactions = self.node.get_transactions(block).await?;
        let tx_count = transactions.len();

        for tx in transactions {
            self.storage.store_transaction(&tx).await?;
        }
        info!(target: "eventify::idx::tx", processed=?true, tx_count=?tx_count, block=?block, elapsed=?now.elapsed());

        Ok(())
    }

    async fn process_transactions_from_range(
        &self,
        from: BlockNumber,
        to: BlockNumber,
    ) -> crate::Result<()> {
        info!(target: "eventify::idx::tx", "Processing transactions from blocks {}..{}", from, to);

        for block in from..=to {
            self.process_transactions(block).await?;
        }

        Ok(())
    }

    async fn process_logs(&self, criteria: &Criteria) -> crate::Result<()> {
        let now = Instant::now();

        let logs = self.node.get_logs(criteria).await?;
        let mut log_count = 0;

        for log in logs {
            self.storage.store_log(&log).await?;
            log_count += 1;

            if log_count % 100 == 0 {
                info!(target: "eventify::idx::logs", processed=?true, log_count=?log_count, latest_tx_hash=?log.transaction_hash, elapsed=?now.elapsed());
            }
        }

        Ok(())
    }

    async fn stream_blocks(&self) -> crate::Result<()> {
        let mut stream = self.node.stream_blocks().await?;

        while let Some(block) = stream.next().await {
            trace!(target: "eventify::core::collector::stream_blocks", "{:#?}", block);
            let block = block?;

            info!(target: "eventify::core::collector::stream_blocks", block_number=?block.number.map(|x| x.to::<u64>()));
            self.storage.store_block(&block).await?;
        }

        Ok(())
    }

    async fn stream_transactions(&self) -> crate::Result<()> {
        let mut stream = self.node.stream_blocks().await?;

        while let Some(block) = stream.next().await {
            trace!(target: "eventify::core::collector::stream_transactions", "{:#?}", block);
            let block = block?;
            let tx = self
                .node
                .get_transactions(block.number.expect("Invalid block number").to::<u64>())
                .await?;
            trace!(target: "eventify::core::collector::stream_transactions", "{:#?}", tx);

            for tx in tx {
                info!(target: "eventify::core::collector::stream_transactions", tx_hash=?tx.hash);
                self.storage.store_transaction(&tx).await?;
            }
        }

        Ok(())
    }

    async fn stream_logs(&self) -> crate::Result<()> {
        let mut stream = self.node.stream_logs().await?;

        while let Some(log) = stream.next().await {
            trace!(target: "eventify::core::collector::stream_logs", "{:#?}", log);
            let log = log?;

            info!(target: "eventify::core::collector::stream_logs", address=?log.address, tx_hash=?log.transaction_hash);
            self.storage.store_log(&log).await?;
        }

        Ok(())
    }
}
