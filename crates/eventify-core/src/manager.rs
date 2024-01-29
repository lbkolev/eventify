use tokio::task::JoinHandle;
use tracing::{error, info};

use crate::{collector::Collector, emit::Emit, provider::Node, Collect, Storage};
use eventify_configs::configs::ManagerConfig;

#[derive(Debug, Clone)]
pub struct Manager<N, S, E>
where
    N: Node,
    S: Storage,
    E: Emit,
{
    pub config: ManagerConfig,
    pub collector: Collector<N, S, E>,
}

impl<N, S, E> Manager<N, S, E>
where
    N: Node,
    S: Storage,
    E: Emit,
{
    pub fn new(config: ManagerConfig, collector: Collector<N, S, E>) -> Self {
        Self { config, collector }
    }
}

impl<N, S, E> Manager<N, S, E>
where
    N: Node,
    S: Storage,
    E: Emit,
{
    pub async fn get_blocks_task(&self) -> crate::Result<JoinHandle<()>> {
        if self.config.skip_blocks {
            return Ok(tokio::spawn(async move {}));
        }

        if let Some(range) = self.config.range.clone() {
            let collector_block = self.collector.clone();
            info!(target: "eventify::core::manager::get_blocks_task", "Spawning a block-getting thread");
            Ok(tokio::spawn(async move {
                match collector_block.process_blocks(range.src, range.dst).await {
                    Ok(_) => {
                        info!(target: "eventify::core::manager::get_blocks_task", "Block-getting thread finished")
                    }
                    Err(e) => {
                        error!(target: "eventify::core::manager::get_blocks_task", "Block-getting thread failed: {}", e)
                    }
                }
            }))
        } else {
            Ok(tokio::spawn(async move {}))
        }
    }

    pub async fn get_transactions_task(&self) -> crate::Result<JoinHandle<()>> {
        if self.config.skip_transactions {
            return Ok(tokio::spawn(async move {}));
        }

        if let Some(range) = self.config.range.clone() {
            let collector_tx = self.collector.clone();
            info!(target: "eventify::core::manager::get_transactions_task", "Spawning a transaction-getting thread");
            Ok(tokio::spawn(async move {
                match collector_tx
                    .process_transactions_from_range(range.src, range.dst)
                    .await
                {
                    Ok(_) => {
                        info!(target: "eventify::core::manager::get_transactions_task", "Transaction-getting thread finished")
                    }
                    Err(e) => {
                        error!(target: "eventify::core::manager::get_transactions_task", "Transaction-getting thread failed: {}", e)
                    }
                }
            }))
        } else {
            Ok(tokio::spawn(async move {}))
        }
    }

    pub async fn get_logs_task(&self) -> crate::Result<JoinHandle<()>> {
        if self.config.skip_logs {
            return Ok(tokio::spawn(async move {}));
        }

        if let Some(criteria) = self.config.criteria.clone() {
            let collector_logs = self.collector.clone();
            info!(target: "eventify::core::manager::get_logs_task", "Spawning a log-getting thread");
            Ok(tokio::spawn(async move {
                match collector_logs.process_logs(&criteria).await {
                    Ok(_) => {
                        info!(target: "eventify::core::manager::get_logs_task", "Log-getting thread finished")
                    }
                    Err(e) => {
                        error!(target: "eventify::core::manager::get_logs_task", "Log-getting thread failed: {}", e)
                    }
                }
            }))
        } else {
            Ok(tokio::spawn(async move {}))
        }
    }

    pub async fn stream_blocks_task(&self) -> crate::Result<JoinHandle<()>> {
        if !self.config.skip_blocks {
            let collector_block = self.collector.clone();
            info!(target: "eventify::core::manager::stream_blocks_task", "Spawning a block-streaming thread");
            Ok(tokio::spawn(async move {
                match collector_block.stream_blocks().await {
                    Ok(_) => {
                        info!(target: "eventify::core::manager::stream_blocks_task", "Block-streaming thread finished")
                    }
                    Err(e) => {
                        error!(target: "eventify::core::manager::stream_blocks_task", "Block-streaming thread failed: {}", e)
                    }
                }
            }))
        } else {
            Ok(tokio::spawn(async move {}))
        }
    }

    pub async fn stream_transactions_task(&self) -> crate::Result<JoinHandle<()>> {
        if !self.config.skip_transactions {
            let collector_tx = self.collector.clone();
            info!(target: "eventify::core::manager::stream_transactions_task", "Spawning a transaction-streaming thread");
            Ok(tokio::spawn(async move {
                match collector_tx.stream_transactions().await {
                    Ok(_) => {
                        info!(target: "eventify::core::manager::stream_transactions_task", "Transaction-streaming thread finished")
                    }
                    Err(e) => {
                        error!(target: "eventify::core::manager::stream_transactions_task", "Transaction-streaming thread failed: {}", e)
                    }
                }
            }))
        } else {
            Ok(tokio::spawn(async move {}))
        }
    }

    pub async fn stream_logs_task(&self) -> crate::Result<JoinHandle<()>> {
        let collector_logs = self.collector.clone();
        info!(target: "eventify::core::manager::stream_logs_task", "Spawning a log-streaming thread");
        Ok(tokio::spawn(async move {
            match collector_logs.stream_logs().await {
                Ok(_) => {
                    info!(target: "eventify::core::manager::stream_logs_task", "Log-streaming thread finished")
                }
                Err(e) => {
                    error!(target: "eventify::core::manager::stream_logs_task", "Log-streaming thread failed: {}", e)
                }
            }
        }))
    }
}
