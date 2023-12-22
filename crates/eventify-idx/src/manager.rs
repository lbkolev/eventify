use alloy_primitives::BlockNumber;

use crate::{
    types::{collect::Collect, provider::NodeProvider},
    Collector, Run,
};
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
        let collector_block = collector.clone();
        let collector_tx = collector.clone();

        let mut handles = vec![
            tokio::spawn(async move {
                let _ = collector_block.process_blocks(src_block, dst_block).await;
            }),
            tokio::spawn(async move {
                let _ = collector_tx
                    .process_transactions_from_range(src_block, dst_block)
                    .await;
            }),
        ];

        if let Some(criterias) = criterias {
            for criteria in criterias {
                let collector_logs = collector.clone();

                handles.push(tokio::spawn(async move {
                    let _ = collector_logs.process_logs(criteria).await;
                }));
            }
        }

        futures::future::join_all(handles).await;
        Ok(())
    }
}
