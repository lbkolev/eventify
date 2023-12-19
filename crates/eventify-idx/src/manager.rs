use alloy_primitives::BlockNumber;

use crate::{types::provider::NodeProvider, Collector, Run};
use eventify_primitives::{Criterias, Storage};

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
        _: Collector<N, S>,
        _: BlockNumber,
        _: BlockNumber,
        _: Option<Criterias>,
    ) -> std::result::Result<(), E>
    where
        E: std::error::Error + Send + Sync,
        N: NodeProvider<crate::Error>,
        S: Storage,
    {
        todo!();
    }

    async fn run_par<N, S, E>(
        collector: Collector<N, S>,
        src_block: BlockNumber,
        dst_block: BlockNumber,
        criterias: Option<Criterias>,
    ) -> std::result::Result<(), E>
    where
        E: std::error::Error + Send + Sync,
        N: NodeProvider<crate::Error>,
        S: Storage,
    {
        let collector_logs = collector.clone();

        let handles = vec![
            tokio::spawn(async move {
                let _ = collector.fetch_blocks(src_block, dst_block).await;
            }),
            tokio::spawn(async move {
                if criterias.is_some() {
                    let _ = collector_logs
                        .fetch_logs(criterias.unwrap(), src_block)
                        .await;
                }
            }),
        ];

        futures::future::join_all(handles).await;
        Ok(())
    }
}
