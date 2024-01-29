use std::time::Instant;

use alloy_primitives::BlockNumber;
use eventify_configs::core::CollectorConfig;
use tracing::{info, trace, error};

use crate::{provider::Node, Collect, Storage, emit::Emit};
use eventify_primitives::Criteria;

#[derive(Debug, Clone)]
pub struct Collector<N, S, E>
where
    N: Node,
    S: Storage,
    E: Emit,
{
    config: CollectorConfig,
    node: N,
    storage: S,
    mid: E
}

impl<N, S, E> Collector<N, S, E>
where
    N: Node,
    S: Storage,
    E: Emit
{
    pub fn new(config: CollectorConfig, node: N, storage: S, mid: E) -> Self {
        Self { config, node, storage, mid }
    }

    pub async fn get_latest_block(&self) -> crate::Result<BlockNumber> {
        self.node.get_block_number().await.map_err(Into::into)
    }
}

impl<N, S, E> Collect<crate::Error> for Collector<N, S, E>
where
    N: Node,
    S: Storage,
    E: Emit
{
    async fn process_block(&self, block: BlockNumber) -> crate::Result<()> {
        let block = self.node.get_block(block).await?;
        self.storage.store_block(&block).await?;
        self.mid.publish(format!("{}:block", self.config.network).as_str(), serde_json::to_string(&block)?)?;

        Ok(())
    }

    async fn process_blocks(&self, from: BlockNumber, to: BlockNumber) -> crate::Result<()> {
        info!(target: "eventify::core::collector::process_blocks", from_block=?from, to_block=?to);
        let now = Instant::now();

        for block in from..=to {
            self.process_block(block).await?;

            if block % 30 == 0 {
                info!(
                    target: "eventify::core::collector::process_blocks", 
                    processed=?true, block_count=?block - from, 
                    latest=?block, elapsed=?now.elapsed());
            }
        }

        Ok(())
    }

    async fn process_transactions(&self, block: BlockNumber) -> crate::Result<()> {
        let now = Instant::now();
        let tx = self.node.get_transactions(block).await?;
        let tx_count = tx.len();

        for tx in tx {
            self.storage.store_transaction(&tx).await?;
            self.mid.publish(format!("{}:transaction", self.config.network).as_str(), serde_json::to_string(&tx).unwrap()).unwrap();
        }
        info!(target: "eventify::idx::tx", processed=?true, tx_count=?tx_count, block=?block, elapsed=?now.elapsed());

        Ok(())
    }

    async fn process_transactions_from_range(
        &self,
        from: BlockNumber,
        to: BlockNumber,
    ) -> crate::Result<()> {
        info!(target: "eventify::core::collector::process_transactions_from_range", "Processing transactions from blocks {}..{}", from, to);

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
            self.mid.publish(format!("{}:log", self.config.network).as_str(), serde_json::to_string(&log).unwrap()).unwrap();
            log_count += 1;

            if log_count % 100 == 0 {
                info!(target: "eventify::core::collector::process_logs", processed=?true, log_count=?log_count, latest_tx_hash=?log.transaction_hash, elapsed=?now.elapsed());
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
            self.mid.publish(format!("{}:block", self.config.network).as_str(), serde_json::to_string(&block).unwrap()).unwrap();
        }

        Ok(())
    }

    async fn stream_transactions(&self) -> crate::Result<()> {
        let mut stream = self.node.stream_blocks().await?;

        while let Some(block) = stream.next().await {
            let block = block?;
            let tx = self
                .node
                .get_transactions(block.number.expect("Invalid block number").to::<u64>())
                .await?;
            trace!(target: "eventify::core::collector::stream_transactions", "{:#?}", tx);

            for tx in tx {
                info!(target: "eventify::core::collector::stream_transactions", tx_hash=?tx.hash);
                self.storage.store_transaction(&tx).await?;
                self.mid.publish(format!("{}:transaction", self.config.network).as_str(), serde_json::to_string(&tx).unwrap()).unwrap();
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
            self.mid.publish(format!("{}:log", self.config.network).as_str(), serde_json::to_string(&log).unwrap()).unwrap();
        }

        Ok(())
    }
}
