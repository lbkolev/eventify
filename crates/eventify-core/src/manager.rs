use alloy_primitives::BlockNumber;
use tokio::{sync::watch::Receiver, task::JoinHandle};
use tracing::{error, info};

use crate::{collector::Collector, emit::Emit, provider::Node, Collect, Storage};
use eventify_configs::configs::ManagerConfig;
use eventify_primitives::{Criteria, ResourceKind};

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
    pub async fn init_collect_tasks(
        &self,
        signal_receiver: Receiver<bool>,
        src: BlockNumber,
        dst: BlockNumber,
        criteria: Criteria,
    ) -> crate::Result<Vec<JoinHandle<()>>> {
        let mut tasks = Vec::new();

        for r in self.config.resources.iter() {
            match r {
                ResourceKind::Block => {
                    tasks.push(
                        self.create_block_collect_task(signal_receiver.clone(), src, dst)
                            .await?,
                    );
                }
                ResourceKind::Transaction => {
                    tasks.push(
                        self.create_transaction_collect_task(signal_receiver.clone(), src, dst)
                            .await?,
                    );
                }
                ResourceKind::Log => {
                    tasks.push(
                        self.create_log_collect_task(signal_receiver.clone(), criteria.clone())
                            .await?,
                    );
                }
            }
        }

        Ok(tasks)
    }

    pub async fn init_stream_tasks(
        &self,
        signal_receiver: Receiver<bool>,
    ) -> crate::Result<Vec<JoinHandle<()>>> {
        let mut tasks = Vec::new();

        for r in self.config.resources.iter() {
            match r {
                ResourceKind::Block => {
                    tasks.push(
                        self.create_block_stream_task(signal_receiver.clone())
                            .await?,
                    );
                }
                ResourceKind::Transaction => {
                    tasks.push(
                        self.create_transaction_stream_task(signal_receiver.clone())
                            .await?,
                    );
                }
                ResourceKind::Log => {
                    tasks.push(self.create_log_stream_task(signal_receiver.clone()).await?);
                }
            }
        }

        Ok(tasks)
    }

    pub async fn create_block_collect_task(
        &self,
        signal_receiver: Receiver<bool>,
        src: BlockNumber,
        dst: BlockNumber,
    ) -> crate::Result<JoinHandle<()>> {
        let collector = self.collector.clone();
        info!("Spawning blocks thread");
        Ok(tokio::spawn(async move {
            match collector.collect_blocks(signal_receiver, src, dst).await {
                Ok(_) => {
                    info!("Blocks thread finished")
                }
                Err(e) => {
                    error!("Blocks thread failed: {}", e)
                }
            }
        }))
    }

    pub async fn create_transaction_collect_task(
        &self,
        signal_receiver: Receiver<bool>,
        src: BlockNumber,
        dst: BlockNumber,
    ) -> crate::Result<JoinHandle<()>> {
        let collector = self.collector.clone();
        info!("Spawning transactions thread");
        Ok(tokio::spawn(async move {
            match collector
                .collect_transactions_from_range(signal_receiver, src, dst)
                .await
            {
                Ok(_) => {
                    info!("Transactions thread finished")
                }
                Err(e) => {
                    error!("Transactions thread failed: {}", e)
                }
            }
        }))
    }

    pub async fn create_log_collect_task(
        &self,
        signal_receiver: Receiver<bool>,
        criteria: Criteria,
    ) -> crate::Result<JoinHandle<()>> {
        let collector = self.collector.clone();
        info!("Spawning logs thread");
        Ok(tokio::spawn(async move {
            match collector.collect_logs(signal_receiver, &criteria).await {
                Ok(_) => {
                    info!("Logs thread finished")
                }
                Err(e) => {
                    error!("Logs thread failed: {}", e)
                }
            }
        }))
    }

    pub async fn create_block_stream_task(
        &self,
        signal_receiver: Receiver<bool>,
    ) -> crate::Result<JoinHandle<()>> {
        let collector = self.collector.clone();
        info!("Spawning block streaming thread");
        Ok(tokio::spawn(async move {
            match collector.stream_blocks(signal_receiver).await {
                Ok(_) => {
                    info!("Block streaming thread finished")
                }
                Err(e) => {
                    error!("Block streaming thread failed: {}", e)
                }
            }
        }))
    }

    pub async fn create_transaction_stream_task(
        &self,
        signal_receiver: Receiver<bool>,
    ) -> crate::Result<JoinHandle<()>> {
        let collector = self.collector.clone();
        info!("Spawning transaction streaming thread");
        Ok(tokio::spawn(async move {
            match collector.stream_transactions(signal_receiver).await {
                Ok(_) => {
                    info!("Transaction streaming thread finished")
                }
                Err(e) => {
                    error!("Transaction streaming thread failed: {}", e)
                }
            }
        }))
    }

    pub async fn create_log_stream_task(
        &self,
        signal_receiver: Receiver<bool>,
    ) -> crate::Result<JoinHandle<()>> {
        let collector = self.collector.clone();
        info!("Spawning log streaming thread");
        Ok(tokio::spawn(async move {
            match collector.stream_logs(signal_receiver).await {
                Ok(_) => {
                    info!("Log streaming thread finished")
                }
                Err(e) => {
                    error!("Log streaming thread failed: {}", e)
                }
            }
        }))
    }
}
