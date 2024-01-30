
use tokio::{sync::watch::Receiver, task::JoinHandle};
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
    pub async fn get_blocks_task(
        &self,
        signal_receiver: Receiver<bool>,
    ) -> crate::Result<JoinHandle<()>> {
        if let Some(range) = self.config.range.clone() {
            let collector = self.collector.clone();
            info!("Spawning a blocks thread");
            Ok(tokio::spawn(async move {
                match collector
                    .process_blocks(signal_receiver, range.src, range.dst)
                    .await
                {
                    Ok(_) => {
                        info!("Blocks thread finished")
                    }
                    Err(e) => {
                        error!("Blocks thread failed: {}", e)
                    }
                }
            }))
        } else {
            Ok(tokio::spawn(async move {}))
        }
    }

    pub async fn get_transactions_task(
        &self,
        signal_receiver: Receiver<bool>,
    ) -> crate::Result<JoinHandle<()>> {
        if let Some(range) = self.config.range.clone() {
            let collector = self.collector.clone();
            info!("Spawning a transactions thread");
            Ok(tokio::spawn(async move {
                match collector
                    .process_transactions_from_range(signal_receiver, range.src, range.dst)
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
        } else {
            Ok(tokio::spawn(async move {}))
        }
    }

    pub async fn get_logs_task(
        &self,
        signal_receiver: Receiver<bool>,
    ) -> crate::Result<JoinHandle<()>> {
        if let Some(criteria) = self.config.criteria.clone() {
            let collector = self.collector.clone();
            info!("Spawning a logs thread");
            Ok(tokio::spawn(async move {
                match collector.process_logs(signal_receiver, &criteria).await {
                    Ok(_) => {
                        info!("Logs thread finished")
                    }
                    Err(e) => {
                        error!("Logs thread failed: {}", e)
                    }
                }
            }))
        } else {
            Ok(tokio::spawn(async move {}))
        }
    }

    pub async fn stream_blocks_task(
        &self,
        signal_receiver: Receiver<bool>,
    ) -> crate::Result<JoinHandle<()>> {
        let collector = self.collector.clone();
        info!("Spawning a block streaming thread");
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

    pub async fn stream_transactions_task(
        &self,
        signal_receiver: Receiver<bool>,
    ) -> crate::Result<JoinHandle<()>> {
        let collector = self.collector.clone();
        info!("Spawning a transaction streaming thread");
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

    pub async fn stream_logs_task(
        &self,
        signal_receiver: Receiver<bool>,
    ) -> crate::Result<JoinHandle<()>> {
        let collector = self.collector.clone();
        info!("Spawning a log streaming thread");
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
