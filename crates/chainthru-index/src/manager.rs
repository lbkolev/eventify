use ethers_providers::JsonRpcClient;

use crate::{BlockProcessor, LogProcessor, Processor, Result, Runner};
use chainthru_primitives::{Auth, Storage};

#[derive(Debug, Clone, Default)]
pub struct Manager;

impl Manager {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl Runner for Manager {
    type Error = crate::Error;

    async fn run<
        T: JsonRpcClient + Clone + Send + Sync,
        U: Storage + Auth + Clone + Send + Sync,
    >(
        processor: Processor<T, U>,
    ) -> std::result::Result<(), Self::Error> {
        processor.process_all_serial().await?;

        Ok(())
    }

    async fn run_par<
        T: JsonRpcClient + Clone + Send + Sync,
        U: Storage + Auth + Clone + Send + Sync,
    >(
        processor: Processor<T, U>,
    ) -> Result<()> {
        let block_processor = processor.clone();
        let log_processor = processor.clone();

        let handles = vec![
            tokio::spawn(async move { log_processor.process_logs().await }),
            tokio::spawn(async move { block_processor.process_blocks().await }),
        ];

        futures::future::join_all(handles).await;

        Ok(())
    }
}
