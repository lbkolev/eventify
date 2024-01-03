use alloy_primitives::BlockNumber;
use tracing::{error, info};

use crate::{
    clients::{NodeClient, StorageClient},
    collector::Collect,
    Collector,
};
use eventify_primitives::Criterias;

use std::error::Error;

#[async_trait::async_trait]
pub trait Run {
    async fn run<N, S, E>(
        processor: Collector<N, S>,
        skip_transactions: bool,
        skip_blocks: bool,
        src_block: BlockNumber,
        dst_block: BlockNumber,
        criterias: Option<Criterias>,
    ) -> Result<(), E>
    where
        E: Error + Send + Sync,
        N: NodeClient,
        S: StorageClient;
}

#[derive(Debug, Clone, Default)]
pub struct Manager;

impl Manager {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl Run for Manager {
    async fn run<N, S, E>(
        collector: Collector<N, S>,
        skip_transactions: bool,
        skip_blocks: bool,
        src_block: BlockNumber,
        dst_block: BlockNumber,
        criterias: Option<Criterias>,
    ) -> std::result::Result<(), E>
    where
        E: std::error::Error + Send + Sync,
        N: NodeClient,
        S: StorageClient,
    {
        let mut handles = vec![];
        if !skip_transactions {
            let collector_tx = collector.clone();
            info!(target: "eventify::idx", src_block=?src_block, dst_block=?dst_block, "Spawning a transaction-processing thread");
            handles.push(tokio::spawn(async move {
                match collector_tx.process_transactions_from_range(src_block, dst_block).await {
                    Ok(_) => info!(target: "eventify::idx", src_block=?src_block, dst_block=?dst_block, "Transaction-processing thread finished"),
                    Err(e) => error!(target: "eventify::idx", src_block=?src_block, dst_block=?dst_block, "Transaction-processing thread failed: {}", e),
                }
            }));
        }

        if !skip_blocks {
            let collector_block = collector.clone();
            info!(target: "eventify::idx", src_block=?src_block, dst_block=?dst_block, "Spawning a block-processing thread");
            handles.push(tokio::spawn(async move {
                match collector_block.process_blocks(src_block, dst_block).await {
                    Ok(_) => info!(target: "eventify::idx", src_block=?src_block, dst_block=?dst_block, "Block-processing thread finished"),
                    Err(e) => error!(target: "eventify::idx", src_block=?src_block, dst_block=?dst_block, "Block-processing thread failed: {}", e),
                }
            }));
        }

        if let Some(criterias) = criterias {
            for criteria in criterias {
                let collector_logs = collector.clone();
                info!(target: "eventify::idx", criteria=?criteria, "Spawning a log-processing thread");

                handles.push(tokio::spawn(async move {
                    match collector_logs.process_logs(criteria).await {
                        Ok(_) => info!(target: "eventify::idx", "Log-processing thread finished"),
                        Err(e) => {
                            error!(target: "eventify::idx", "Log-processing thread failed: {}", e)
                        }
                    }
                }));
            }
        }

        futures::future::join_all(handles).await;
        Ok(())
    }
}
