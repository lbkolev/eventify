use std::{
    time::Instant,
};

use alloy_primitives::BlockNumber;

use tokio::sync::watch::Receiver;
use tracing::{info, trace};

use crate::{emit::Emit, provider::Node, Collect, Storage};
use eventify_configs::core::CollectorConfig;
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
    mid: E,
}

impl<N, S, E> Collector<N, S, E>
where
    N: Node,
    S: Storage,
    E: Emit,
{
    pub fn new(config: CollectorConfig, node: N, storage: S, mid: E) -> Self {
        Self {
            config,
            node,
            storage,
            mid,
        }
    }

    pub async fn get_latest_block(&self) -> crate::Result<BlockNumber> {
        self.node.get_block_number().await.map_err(Into::into)
    }
}

impl<N, S, E> Collect<crate::Error> for Collector<N, S, E>
where
    N: Node,
    S: Storage,
    E: Emit,
{
    async fn process_block(&self, block: BlockNumber) -> crate::Result<()> {
        let block = self.node.get_block(block).await?;
        self.storage.store_block(&block).await?;
        self.mid.publish(
            format!("{}:block", self.config.network).as_str(),
            serde_json::to_string(&block)?,
        )?;

        Ok(())
    }

    async fn process_blocks(
        &self,
        signal_receiver: Receiver<bool>,
        from: BlockNumber,
        to: BlockNumber,
    ) -> crate::Result<()> {
        info!(from_block=?from, to_block=?to);
        let now = Instant::now();

        for block in from..=to {
            if signal_receiver.borrow().to_owned() {
                trace!("Received a signal to stop processing blocks");
                break;
            };

            self.process_block(block).await?;
            if block % 30 == 0 {
                info!(
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
            self.mid.publish(
                format!("{}:transaction", self.config.network).as_str(),
                serde_json::to_string(&tx)?,
            )?;
        }
        info!(processed=?true, tx_count=?tx_count, block=?block, elapsed=?now.elapsed());

        Ok(())
    }

    async fn process_transactions_from_range(
        &self,
        signal_receiver: Receiver<bool>,
        from: BlockNumber,
        to: BlockNumber,
    ) -> crate::Result<()> {
        info!("Processing transactions from blocks {}..{}", from, to);

        for block in from..=to {
            if signal_receiver.borrow().to_owned() {
                trace!("Received a signal to stop processing transactions");
                break;
            };

            self.process_transactions(block).await?;
        }

        Ok(())
    }

    async fn process_logs(
        &self,
        signal_receiver: Receiver<bool>,
        criteria: &Criteria,
    ) -> crate::Result<()> {
        let now = Instant::now();

        let logs = self.node.get_logs(criteria).await?;
        let mut log_count = 0;

        for log in logs {
            if signal_receiver.borrow().to_owned() {
                trace!("Received a signal to stop processing logs");
                break;
            };

            self.storage.store_log(&log).await?;
            self.mid.publish(
                format!("{}:log", self.config.network).as_str(),
                serde_json::to_string(&log)?,
            )?;
            log_count += 1;

            if log_count % 100 == 0 {
                info!(processed=?true, log_count=?log_count, latest_tx_hash=?log.transaction_hash, elapsed=?now.elapsed());
            }
        }

        Ok(())
    }

    async fn stream_blocks(&self, signal_receiver: Receiver<bool>) -> crate::Result<()> {
        let mut stream = self.node.stream_blocks().await?;

        while let Some(block) = stream.next().await {
            if signal_receiver.borrow().to_owned() {
                trace!("Received a signal to stop streaming blocks");
                break;
            };

            let block = block?;
            trace!(block=?block);
            info!(r#type="block", number=?block.number, hash=?block.hash);
            self.storage.store_block(&block).await?;
            self.mid.publish(
                format!("{}:block", self.config.network).as_str(),
                serde_json::to_string(&block)?,
            )?;
        }

        Ok(())
    }

    async fn stream_transactions(&self, signal_receiver: Receiver<bool>) -> crate::Result<()> {
        let mut stream = self.node.stream_blocks().await?;

        while let Some(block) = stream.next().await {
            if signal_receiver.borrow().to_owned() {
                trace!("Received a signal to stop streaming transactions");
                break;
            };

            let block = block?;
            let tx = self
                .node
                .get_transactions(block.number.expect("Invalid block number").to::<u64>())
                .await?;
            for tx in tx {
                trace!(tx=?tx);
                info!(r#type="tx", hash=?tx.hash);
                self.storage.store_transaction(&tx).await?;
                self.mid.publish(
                    format!("{}:transaction", self.config.network).as_str(),
                    serde_json::to_string(&tx)?,
                )?;
            }
        }

        Ok(())
    }

    async fn stream_logs(&self, signal_receiver: Receiver<bool>) -> crate::Result<()> {
        let mut stream = self.node.stream_logs().await?;

        while let Some(log) = stream.next().await {
            if signal_receiver.borrow().to_owned() {
                trace!("Received a signal to stop streaming logs");
                break;
            };

            let log = log?;
            trace!(log=?log);
            info!(r#type="log", address=?log.address, tx_hash=?log.transaction_hash);
            self.storage.store_log(&log).await?;
            self.mid.publish(
                format!("{}:log", self.config.network).as_str(),
                serde_json::to_string(&log)?,
            )?;
        }

        Ok(())
    }
}
