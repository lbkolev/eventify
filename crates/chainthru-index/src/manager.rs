use ethers_providers::JsonRpcClient;
use tokio::task::JoinHandle;

use crate::{App, BlockProcessor, LogProcessor, Processor, Result, Runner};
use chainthru_primitives::{Auth, Criterias, Storage};

pub struct Manager;

impl Manager {
    pub fn new() -> Self {
        Self
    }
}

impl Runner for Manager {
    type Error = crate::Error;

    async fn run<
        T: JsonRpcClient + Clone + Send + Sync,
        U: Storage + Auth + Clone + Send + Sync,
    >(
        app: Processor<T, U>,
    ) -> std::result::Result<(), Self::Error> {
        todo!()
    }

    #[cfg(feature = "multi-thread")]
    async fn run_par<
        T: JsonRpcClient + Clone + Send + Sync,
        U: Storage + Auth + Clone + Send + Sync,
    >(
        app: Processor<T, U>,
    ) -> Result<()> {
        let block_processor = app.clone();
        let log_processor = app.clone();

        let handles = vec![
            tokio::spawn(async move { log_processor.process_logs().await }),
            tokio::spawn(async move { block_processor.process_blocks().await }),
        ];

        futures::future::join_all(handles).await;

        Ok(())
    }
}
