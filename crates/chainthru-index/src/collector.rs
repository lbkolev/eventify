use async_trait::async_trait;
use ethers_providers::JsonRpcClient;

use crate::{App, Process, Result};
use chainthru_primitives::{Auth, Criterias, IndexedBlock, IndexedLog, Storage};

#[derive(Debug, Clone)]
pub struct Collector<T, U>
where
    T: JsonRpcClient + Clone + 'static,
    U: Storage + Auth + Clone + 'static,
{
    app: App<T, U>,
    criterias: Option<Criterias>,
}

impl<T, U> Collector<T, U>
where
    T: JsonRpcClient + Clone + 'static,
    U: Storage + Auth + Clone + 'static,
{
    pub fn new(app: App<T, U>, criterias: Option<Criterias>) -> Self {
        Self { app, criterias }
    }

    pub async fn process_all_serial(&self) -> Result<()> {
        let from = self.app.src_block();
        let to = self.app.dst_block();

        for target in from..=to {
            if let Some(crits) = self.criterias.as_ref() {
                let logs = self.app.fetch_logs(crits, target).await?;
                log::info!("{:?}", logs);

                for log in logs {
                    println!("{:#?}", log);
                    self.app.storage_conn()?.insert_log(&log.into()).await?;
                }
            }

            let (block, transactions) = match self.app.fetch_indexed_data(target).await {
                Ok((block, transactions)) => (block, transactions),
                Err(_) => {
                    // TODO: impl stream subscription
                    if target >= self.app.get_latest_block().await? {
                        log::info!("Reached latest block: {:?}", target);
                        break;
                    }

                    continue;
                }
            };

            self.app.storage_conn()?.insert_block(&block).await?;
            for tx in transactions {
                if tx.contract_creation() {
                    self.app
                        .storage_conn()?
                        .insert_contract(&tx.clone().into())
                        .await?;
                }
                self.app.storage_conn()?.insert_transaction(&tx).await?;
            }
        }

        Ok(())
    }
}

#[async_trait]
impl<T, U> Process<IndexedBlock> for Collector<T, U>
where
    T: JsonRpcClient + Clone + 'static,
    U: Storage + Auth + Clone + 'static,
{
    type Error = crate::Error;

    async fn process(&self) -> Result<()> {
        // TODO: proper err handling
        let from = self.app.src_block();
        let to = self.app.dst_block();

        for target in from..=to {
            let (block, transactions) = match self.app.fetch_indexed_data(target).await {
                Ok((block, transactions)) => (block, transactions),
                Err(_) => {
                    // TODO: impl stream subscription
                    if target >= self.app.get_latest_block().await? {
                        log::info!("Reached latest block: {:?}", target);
                        break;
                    }
                    continue;
                }
            };

            self.app.storage_conn()?.insert_block(&block).await?;
            for tx in transactions {
                if tx.contract_creation() {
                    self.app
                        .storage_conn()?
                        .insert_contract(&tx.clone().into())
                        .await?;
                }
                self.app.storage_conn()?.insert_transaction(&tx).await?;
            }
        }

        Ok(())
    }

    async fn stream(&self) -> Result<()> {
        todo!()
    }

    async fn stream_latest(&self) -> Result<()> {
        todo!()
    }
}

#[async_trait]
impl<T, U> Process<IndexedLog> for Collector<T, U>
where
    T: JsonRpcClient + Clone + 'static,
    U: Storage + Auth + Clone + 'static,
{
    type Error = crate::Error;

    async fn process(&self) -> Result<()> {
        // TODO: proper err handling
        let from = self.app.src_block();
        let to = self.app.dst_block();

        for target in from..=to {
            if let Some(crits) = self.criterias.as_ref() {
                let logs = self.app.fetch_logs(crits, target).await.unwrap();
                log::info!("{:?}", logs);

                for log in logs {
                    println!("{:#?}", log);
                    self.app
                        .storage_conn()
                        .unwrap()
                        .insert_log(&log.into())
                        .await
                        .unwrap();
                }
            }
        }

        Ok(())
    }

    async fn stream(&self) -> Result<()> {
        todo!()
    }

    async fn stream_latest(&self) -> Result<()> {
        todo!()
    }
}
