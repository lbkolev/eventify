use alloy_primitives::BlockNumber;
use chainthru_primitives::IndexedTransaction;

use ethers_core::types::{Block, BlockId, Transaction};
use ethers_providers::{Http, Ipc, JsonRpcClient, Middleware, Provider as NodeProvider, Ws};

use crate::Result;
use chainthru_primitives::{
    block::IndexedBlock,
    storage::{Auth, Storage},
};

#[derive(Clone, Debug)]
pub struct App<T: JsonRpcClient, U: Storage + Auth> {
    inner: Providers<T, U>,

    pub(crate) src_block: BlockNumber,
    pub(crate) dst_block: BlockNumber,
}

impl<T: JsonRpcClient, U: Storage + Auth> Default for App<T, U> {
    fn default() -> Self {
        Self {
            inner: Providers::default(),
            src_block: 0,
            dst_block: BlockNumber::MAX,
        }
    }
}

impl<T: JsonRpcClient, U: Storage + Auth> App<T, U> {
    /// Create a new instance of the indexer
    pub fn new(
        transport_node: Option<NodeProvider<T>>,
        transport_storage: Option<U>,
        src_block: BlockNumber,
        dst_block: BlockNumber,
    ) -> Self {
        Self {
            inner: Providers::new(transport_node, transport_storage),
            src_block,
            dst_block,
        }
    }

    pub fn with_src_block(self, src_block: BlockNumber) -> Self {
        Self { src_block, ..self }
    }

    pub fn with_dst_block(self, dst_block: BlockNumber) -> Self {
        Self { dst_block, ..self }
    }

    pub fn with_node_conn(self, transport: NodeProvider<T>) -> Self {
        Self {
            inner: Providers::new(Some(transport), self.inner.transport_storage),
            ..self
        }
    }

    pub fn with_storage(self, url: &str) -> Self {
        Self {
            inner: Providers::new(self.inner.transport_node, Some(U::connect_lazy(url))),
            ..self
        }
    }

    /// Get block details with full transaction objects
    async fn fetch_block(&self, block: BlockId) -> Result<Block<Transaction>> {
        self.inner
            .transport_node
            .as_ref()
            .expect("Unable to get transport node")
            .get_block_with_txs(block)
            .await?
            .ok_or(crate::Error::FetchBlock(format!(
                "Unable to fetch block {:?}",
                block
            )))
    }

    /// Get indexed block & transaction objects from a given block number
    pub async fn fetch_indexed_data(
        &self,
        block: BlockNumber,
    ) -> Result<(IndexedBlock, Vec<IndexedTransaction>)> {
        let block = self
            .fetch_block(BlockId::Number(ethers_core::types::BlockNumber::Number(
                block.into(),
            )))
            .await?;

        let transactions = block
            .clone()
            .transactions
            .into_iter()
            .map(IndexedTransaction::from)
            .collect();

        Ok((IndexedBlock::from(block), transactions))
    }

    // /// Get the transaction receipt for a given transaction hash
    //pub async fn fetch_transaction_receipt(
    //    &self,
    //    transaction_hash: H256,
    //) -> Option<TransactionReceipt> {
    //    std::boxed::Box::into_inner(std::pin::Pin::into_inner(
    //        self.inner
    //            .transport_node
    //            .as_ref()
    //            .expect("Unable to get transport node")
    //            .get_transaction_receipt(transaction_hash),
    //    ))
    //    .await?
    //
    //    //.transaction_receipt(transaction_hash)
    //    //.await
    //    //.unwrap_or(None)
    //}
    //

    /// Returns the latests finalized block number
    pub async fn latest_block(&self) -> Result<u64> {
        Ok(self
            .inner
            .transport_node
            .as_ref()
            .unwrap()
            .get_block_number()
            .await
            .unwrap()
            .as_u64())
    }

    //pub async fn process_contract(&self, transaction: IndexedTransaction) -> Result<()> {
    //    if let Some(receipt) = self.fetch_transaction_receipt(transaction.hash()).await {
    //        let contract = Contract {
    //            address: receipt
    //                .contract_address
    //                .expect("Unable to get contract address"),
    //            transaction_hash: receipt.transaction_hash,
    //            from: transaction
    //                ._from()
    //                .expect("Unable to get transaction sender"),
    //            input: transaction.input().clone(),
    //        };
    //
    //        //match self.storage_conn().insert_contract(&contract).await {
    //        //    Ok(_) => log::info!("Contract inserted"),
    //        //    Err(e) => log::warn!("Error inserting contract: {:?}", e),
    //        //}
    //    }
    //
    //    Ok(())
    //}

    pub fn storage_conn(&self) -> &U {
        self.inner
            .transport_storage
            .as_ref()
            .expect("Unable to get transport db")
    }

    pub fn src_block(&self) -> u64 {
        self.src_block
    }

    pub fn dst_block(&self) -> u64 {
        self.dst_block
    }
}

impl<U: Storage + Auth> App<Http, U> {
    /// Creates a new instance of the App with the HTTP transport
    pub fn with_http(self, node_url: &str) -> Self {
        Self {
            inner: Providers::new(
                Some(NodeProvider::new(Http::new(
                    url::Url::parse(node_url).unwrap(),
                ))),
                self.inner.transport_storage,
            ),
            ..self
        }
    }
}

impl<U: Storage + Auth> App<Ipc, U> {
    /// Creates a new instance of the App with the IPC transport
    pub async fn with_ipc(self, node_url: &str) -> Self {
        Self {
            inner: Providers::new(
                Some(NodeProvider::new(
                    Ipc::connect(node_url)
                        .await
                        .expect("Failed to create IPC transport"),
                )),
                self.inner.transport_storage,
            ),
            ..self
        }
    }
}

impl<U: Storage + Auth> App<Ws, U> {
    /// Creates a new instance of the App with the WebSocket transport
    pub async fn with_websocket(self, node_url: &str) -> Self {
        Self {
            inner: Providers::new(
                Some(NodeProvider::new(
                    Ws::connect(node_url)
                        .await
                        .expect("Failed to create WS transport"),
                )),
                self.inner.transport_storage,
            ),
            ..self
        }
    }

    /// Creates a new instance of the App with the WebSocket transport
    ///
    /// An alias for `with_websocket`
    pub async fn with_ws(self, node_url: &str) -> Self {
        self.with_websocket(node_url).await
    }
}

#[derive(Clone, Debug)]
struct Providers<T: JsonRpcClient, U: Storage + Auth> {
    transport_node: Option<NodeProvider<T>>,
    transport_storage: Option<U>,
}

impl<T: JsonRpcClient, U: Storage + Auth> Default for Providers<T, U> {
    fn default() -> Self {
        Self {
            transport_node: None,
            transport_storage: None,
        }
    }
}

impl<T: JsonRpcClient, U: Storage + Auth> Providers<T, U> {
    pub fn new(node: Option<NodeProvider<T>>, db: Option<U>) -> Self {
        Self {
            transport_node: node,
            transport_storage: db,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_default_app() {
        let app: App<Http, chainthru_primitives::storage::Postgres> = crate::App::default();

        assert!(app.inner.transport_node.is_none());
        assert!(app.inner.transport_storage.is_none());
        assert_eq!(app.src_block, 0);
        assert_eq!(app.dst_block, BlockNumber::MAX);
    }

    #[tokio::test]
    async fn test_transport_https() {
        let app: App<Http, chainthru_primitives::storage::Postgres> = crate::App::default()
            .with_http(
                env::var("CHAINTHRU_TEST_HTTPS_PROVIDER")
                    .unwrap_or("https://eth.llamarpc.com".to_string())
                    .as_str(),
            )
            .with_storage(
                env::var("CHAINTHRU_TEST_DATABASE_URL")
                    .unwrap_or("postgres://postgres:password@localhost:5432/chainthru".to_string())
                    .as_str(),
            );

        assert!(app.inner.transport_node.is_some());
        assert!(app.inner.transport_storage.is_some());
        assert_eq!(app.src_block, 0);
        assert_eq!(app.dst_block, BlockNumber::MAX);
    }

    #[tokio::test]
    async fn test_transport_wss() {
        let app: App<Ws, chainthru_primitives::storage::Postgres> = crate::App::default()
            .with_ws(
                env::var("CHAINTHRU_TEST_WSS_PROVIDER")
                    .unwrap_or("wss://eth.llamarpc.com".to_string())
                    .as_str(),
            )
            .await
            .with_storage(
                env::var("CHAINTHRU_TEST_DATABASE_URL")
                    .unwrap_or("postgres://postgres:password@localhost:5432/chainthru".to_string())
                    .as_str(),
            );

        assert!(app.inner.transport_node.is_some());
        assert!(app.inner.transport_storage.is_some());
        assert_eq!(app.src_block, 0);
        assert_eq!(app.dst_block, BlockNumber::MAX);
    }
}
