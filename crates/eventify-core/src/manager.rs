use alloy_primitives::BlockNumber;
use tokio::{sync::watch::Receiver, task::JoinHandle};
use tracing::{error, info};

use crate::{
    collector::Collector,
    traits::{Collect, Emit, Network, Store},
};
use eventify_configs::configs::ManagerConfig;
use eventify_primitives::networks::{eth::Criteria, ResourceKind};

#[derive(Debug, Clone)]
pub struct Manager<N, S, E>
where
    N: Network,
    S: Store<N>,
    E: Emit<N>,
{
    pub config: ManagerConfig,
    pub collector: Collector<N, S, E>,
}

impl<N, S, E> Manager<N, S, E>
where
    N: Network,
    S: Store<N>,
    E: Emit<N>,
{
    pub fn new(config: ManagerConfig, collector: Collector<N, S, E>) -> Self {
        Self { config, collector }
    }
}

impl<N, S, E> Manager<N, S, E>
where
    N: Network,
    S: Store<N>,
    E: Emit<N>,
{
    pub async fn init_collect_tasks(
        &self,
        stop_signal: Receiver<bool>,
        src: BlockNumber,
        dst: BlockNumber,
        criteria: Criteria,
    ) -> crate::Result<Vec<JoinHandle<()>>> {
        let mut tasks = Vec::new();

        for r in self.config.resources.iter() {
            match r {
                ResourceKind::Block => {
                    tasks.push(
                        self.create_block_collect_task(stop_signal.clone(), src, dst)
                            .await?,
                    );
                }
                ResourceKind::Transaction => {
                    tasks.push(
                        self.create_transaction_collect_task(stop_signal.clone(), src, dst)
                            .await?,
                    );
                }
                ResourceKind::Log(_) => {
                    tasks.push(
                        self.create_log_collect_task(stop_signal.clone(), criteria.clone())
                            .await?,
                    );
                }
            }
        }

        Ok(tasks)
    }

    pub async fn init_stream_tasks(
        &self,
        stop_signal: Receiver<bool>,
    ) -> crate::Result<Vec<JoinHandle<()>>> {
        let mut tasks = Vec::new();

        for r in self.config.resources.iter() {
            match r {
                ResourceKind::Block => {
                    tasks.push(self.create_block_stream_task(stop_signal.clone()).await?);
                }
                ResourceKind::Transaction => {
                    tasks.push(
                        self.create_transaction_stream_task(stop_signal.clone())
                            .await?,
                    );
                }
                ResourceKind::Log(_) => {
                    tasks.push(self.create_log_stream_task(stop_signal.clone()).await?);
                }
            }
        }

        Ok(tasks)
    }

    pub async fn create_block_collect_task(
        &self,
        stop_signal: Receiver<bool>,
        src: BlockNumber,
        dst: BlockNumber,
    ) -> crate::Result<JoinHandle<()>> {
        let collector = self.collector.clone();
        info!("Spawning blocks thread");
        Ok(tokio::spawn(async move {
            let _ = collector.collect_blocks(stop_signal, src, dst).await;
            // std::future::pending::<()>().await
        }))
    }

    pub async fn create_transaction_collect_task(
        &self,
        stop_signal: Receiver<bool>,
        src: BlockNumber,
        dst: BlockNumber,
    ) -> crate::Result<JoinHandle<()>> {
        let collector = self.collector.clone();
        info!("Spawning transactions thread");
        Ok(tokio::spawn(async move {
            let _ = collector
                .collect_transactions_from_range(stop_signal, src, dst)
                .await;
            // std::future::pending::<()>().await
        }))
    }

    pub async fn create_log_collect_task(
        &self,
        stop_signal: Receiver<bool>,
        criteria: Criteria,
    ) -> crate::Result<JoinHandle<()>> {
        let collector = self.collector.clone();
        info!("Spawning logs thread");
        Ok(tokio::spawn(async move {
            let _ = collector.collect_logs(stop_signal, &criteria).await;
            // std::future::pending::<()>().await
        }))
    }

    pub async fn create_block_stream_task(
        &self,
        stop_signal: Receiver<bool>,
    ) -> crate::Result<JoinHandle<()>> {
        let collector = self.collector.clone();
        info!("Spawning block streaming thread");
        Ok(tokio::spawn(async move {
            let _ = collector.stream_blocks(stop_signal).await;
            //std::future::pending::<()>().await
        }))
    }

    pub async fn create_transaction_stream_task(
        &self,
        stop_signal: Receiver<bool>,
    ) -> crate::Result<JoinHandle<()>> {
        let collector = self.collector.clone();
        info!("Spawning transaction streaming thread");
        Ok(tokio::spawn(async move {
            let _ = collector.stream_transactions(stop_signal).await;
            //std::future::pending::<()>().await
        }))
    }

    pub async fn create_log_stream_task(
        &self,
        stop_signal: Receiver<bool>,
    ) -> crate::Result<JoinHandle<()>> {
        let collector = self.collector.clone();
        info!("Spawning log streaming thread");
        Ok(tokio::spawn(async move {
            let _ = collector.stream_logs(stop_signal).await;
            //std::future::pending::<()>().await
        }))
    }
}
