use tokio::{sync::watch::Receiver, task::JoinHandle};
use tracing::{error, info, warn};

use crate::{
    collector::Collector,
    traits::{Collect, Network},
};
use eventify_configs::configs::ManagerConfig;
use eventify_primitives::networks::ResourceKind;

#[derive(Debug, Clone)]
pub struct Manager<N>
where
    N: Network,
{
    pub config: ManagerConfig,
    pub collector: Collector<N>,
}

impl<N> Manager<N>
where
    N: Network,
{
    pub fn new(config: ManagerConfig, collector: Collector<N>) -> Self {
        Self { config, collector }
    }
}

impl<N> Manager<N>
where
    N: Network,
{
    pub async fn init_stream_tasks(
        &self,
        stop_signal: Receiver<bool>,
    ) -> crate::Result<Vec<JoinHandle<()>>> {
        let mut tasks = Vec::new();

        for r in self.config.resources.iter() {
            match r {
                ResourceKind::Block => {
                    let collector = self.collector.clone();
                    let signal = stop_signal.clone();
                    tasks.push(self.create_block_stream_task(collector, signal).await?);
                }
                ResourceKind::Transaction => {
                    let collector = self.collector.clone();
                    let signal = stop_signal.clone();
                    tasks.push(self.create_tx_stream_task(collector, signal).await?);
                }
                ResourceKind::Log(_) => {
                    let collector = self.collector.clone();
                    let signal = stop_signal.clone();
                    tasks.push(self.create_log_stream_task(collector, signal).await?);
                }
            }
        }

        Ok(tasks)
    }

    pub async fn create_block_stream_task(
        &self,
        collector: Collector<N>,
        mut stop_signal: Receiver<bool>,
    ) -> crate::Result<JoinHandle<()>> {
        info!("Spawning block streaming thread");
        Ok(tokio::spawn(async move {
            loop {
                tokio::select! {
                    result = collector.stream_blocks() => {
                        match result {
                            Ok(()) => {
                                info!(thread="stream_logs", "Finished streaming blocks");
                                break;
                            }
                            Err(err) => match err {
                                crate::Error::JsonRpsee(err)  => {
                                    warn!(err=?err);
                                    continue;
                                }
                                crate::Error::EmptyStream => {
                                    warn!("Empty stream");
                                    continue;
                                }
                                _ => {
                                    error!(err=?err);
                                    break;
                                }
                            }
                        }
                    }
                    _ = stop_signal.changed() => {
                        warn!(thread="stream_block", "SIGINT signal. Terminating..");
                        break;
                    }
                }
            }
        }))
    }

    pub async fn create_tx_stream_task(
        &self,
        collector: Collector<N>,
        mut stop_signal: Receiver<bool>,
    ) -> crate::Result<JoinHandle<()>> {
        info!("Spawning tx streaming thread");
        Ok(tokio::spawn(async move {
            loop {
                tokio::select! {
                    result = collector.stream_txs() => {
                        match result {
                            Ok(()) => {
                                info!(thread="stream_logs", "Finished streaming txs");
                                break;
                            }
                            Err(err) => match err {
                                crate::Error::JsonRpsee(err)  => {
                                    warn!(err=?err);
                                    continue;
                                }
                                crate::Error::EmptyStream => {
                                    warn!(err="Empty stream");
                                    continue;
                                }
                                _ => {
                                    error!(err=?err);
                                    break;
                                }
                            }
                        }
                    }
                    _ = stop_signal.changed() => {
                        warn!(thread="stream_txs", "SIGINT signal. Terminating..");
                        break;
                    }
                }
            }
        }))
    }

    pub async fn create_log_stream_task(
        &self,
        collector: Collector<N>,
        mut stop_signal: Receiver<bool>,
    ) -> crate::Result<JoinHandle<()>> {
        info!("Spawning log streaming thread");
        Ok(tokio::spawn(async move {
            loop {
                tokio::select! {
                    result = collector.stream_logs() => {
                        match result {
                            Ok(()) => {
                                info!(thread="stream_logs", "Finished streaming logs");
                                break;
                            }
                            Err(err) => match err {
                                crate::Error::JsonRpsee(err)  => {
                                    warn!(err=?err);
                                    continue;
                                }
                                crate::Error::EmptyStream => {
                                    warn!(err="Empty stream");
                                    continue;
                                }
                                _ => {
                                    error!(err=?err);
                                    break;
                                }
                            }
                        }
                    }
                    _ = stop_signal.changed() => {
                        warn!(thread="stream_logs", "SIGINT signal. Terminating..");
                        break;
                    }
                }
            }
        }))
    }
}
