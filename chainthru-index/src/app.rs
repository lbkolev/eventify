use chainthru_primitives::IndexedTransaction;
use web3::{
    transports::{ipc::Ipc, ws::WebSocket, Http},
    types::{Block, BlockId, BlockNumber, Transaction, H256},
    Transport, Web3,
};

use crate::Result;
use chainthru_primitives::{
    block::IndexedBlock,
    contract::Contract,
    storage::{Auth, Storage},
};

#[derive(Clone, Debug)]
pub struct App<T: Transport, U: Storage + Auth> {
    inner: Providers<T, U>,

    pub(crate) src_block: BlockId,
    pub(crate) dst_block: BlockId,
}

impl<T: Transport, U: Storage + Auth> Default for App<T, U> {
    fn default() -> Self {
        Self {
            inner: Providers::default(),
            src_block: BlockId::Number(BlockNumber::Earliest),
            dst_block: BlockId::Number(BlockNumber::Latest),
        }
    }
}

impl<T: Transport, U: Storage + Auth> App<T, U> {
    /// Create a new instance of the indexer
    pub fn new(
        transport_node: Option<Web3<T>>,
        transport_storage: Option<U>,
        src_block: BlockId,
        dst_block: BlockId,
    ) -> Self {
        Self {
            inner: Providers::new(transport_node, transport_storage),
            src_block,
            dst_block,
        }
    }

    pub fn with_src_block(self, src_block: BlockId) -> Self {
        Self { src_block, ..self }
    }

    pub fn with_dst_block(self, dst_block: BlockId) -> Self {
        Self { dst_block, ..self }
    }

    pub fn with_node_conn(self, transport: Web3<T>) -> Self {
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
    async fn fetch_block(&self, block: BlockId) -> Option<Block<Transaction>> {
        self.inner
            .transport_node
            .as_ref()
            .expect("Unable to get transport node")
            .eth()
            .block_with_txs(block)
            .await
            .unwrap_or(None)
    }

    /// Get indexed block & transaction objects from a given block number
    pub async fn fetch_indexed_data(
        &self,
        block: BlockId,
    ) -> Option<(IndexedBlock, Vec<IndexedTransaction>)> {
        let block = self.fetch_block(block).await?;
        let transactions = block
            .clone()
            .transactions
            .into_iter()
            .map(IndexedTransaction::from)
            .collect();

        Some((IndexedBlock::from(block), transactions))
    }

    /// Get the transaction receipt for a given transaction hash
    pub async fn fetch_transaction_receipt(
        &self,
        transaction_hash: H256,
    ) -> Option<web3::types::TransactionReceipt> {
        self.inner
            .transport_node
            .as_ref()
            .expect("Unable to get transport node")
            .eth()
            .transaction_receipt(transaction_hash)
            .await
            .unwrap_or(None)
    }

    /// Returns the latests finalized block number
    pub async fn latest_block(&self) -> Result<u64> {
        Ok(self
            .inner
            .transport_node
            .as_ref()
            .unwrap()
            .eth()
            .block_number()
            .await?
            .as_u64())
    }

    pub async fn process_contract(&self, transaction: IndexedTransaction) -> Result<()> {
        if let Some(receipt) = self.fetch_transaction_receipt(transaction.hash).await {
            let contract = Contract {
                address: receipt
                    .contract_address
                    .expect("Unable to get contract address"),
                transaction_hash: receipt.transaction_hash,
                from: transaction.from.expect("Unable to get transaction sender"),
                input: transaction.input,
            };

            match self.storage_conn().insert_contract(&contract).await {
                Ok(_) => log::info!("Contract inserted"),
                Err(e) => log::warn!("Error inserting contract: {:?}", e),
            }
        }

        Ok(())
    }

    pub fn storage_conn(&self) -> &U {
        self.inner
            .transport_storage
            .as_ref()
            .expect("Unable to get transport db")
    }

    pub fn src_block(&self) -> u64 {
        match self.src_block {
            BlockId::Number(block) => match block {
                BlockNumber::Number(block) => block.as_u64(),
                BlockNumber::Earliest => 0,
                _ => unimplemented!("Unsupported block type: {:?}", block),
            },
            _ => unimplemented!("Block hash as a src block is not supported yet"),
        }
    }

    pub async fn dst_block(&self) -> Result<u64> {
        match self.dst_block {
            BlockId::Number(block) => match block {
                BlockNumber::Number(block) => Ok(block.as_u64()),
                BlockNumber::Latest => self.latest_block().await,
                _ => unimplemented!("Unsupported block type: {:?}", block),
            },
            _ => unimplemented!("Block hash as a dst block is not supported yet"),
        }
    }
}

impl<U: Storage + Auth> App<Http, U> {
    /// Creates a new instance of the App with the HTTP transport
    pub fn with_http(self, node_url: &str) -> Self {
        Self {
            inner: Providers::new(
                Some(Web3::new(
                    Http::new(node_url).expect("Failed to create HTTP transport"),
                )),
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
                Some(Web3::new(
                    Ipc::new(node_url)
                        .await
                        .expect("Failed to create IPC transport"),
                )),
                self.inner.transport_storage,
            ),
            ..self
        }
    }
}

impl<U: Storage + Auth> App<WebSocket, U> {
    /// Creates a new instance of the App with the WebSocket transport
    pub async fn with_websocket(self, node_url: &str) -> Self {
        Self {
            inner: Providers::new(
                Some(Web3::new(
                    WebSocket::new(node_url)
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
struct Providers<T: Transport, U: Storage + Auth> {
    transport_node: Option<Web3<T>>,
    transport_storage: Option<U>,
}

impl<T: Transport, U: Storage + Auth> Default for Providers<T, U> {
    fn default() -> Self {
        Self {
            transport_node: None,
            transport_storage: None,
        }
    }
}

impl<T: Transport, U: Storage + Auth> Providers<T, U> {
    pub fn new(node: Option<Web3<T>>, db: Option<U>) -> Self {
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
        let app: App<web3::transports::Http, chainthru_primitives::storage::Postgres> =
            crate::App::default();

        assert!(app.inner.transport_node.is_none());
        assert!(app.inner.transport_storage.is_none());
        assert_eq!(app.src_block, BlockId::Number(BlockNumber::Earliest));
        assert_eq!(app.dst_block, BlockId::Number(BlockNumber::Latest));
    }

    #[tokio::test]
    async fn test_transport_https() {
        let app: App<Http, chainthru_primitives::storage::Postgres> = crate::App::default()
            .with_http(
                env::var("CHAINTHRU_TEST_HTTPS_PROVIDER")
                    .unwrap_or(format!("https://eth.llamarpc.com"))
                    .as_str(),
            )
            .with_storage(
                env::var("CHAINTHRU_TEST_DATABASE_URL")
                    .unwrap_or(format!(
                        "postgres://postgres:password@localhost:5432/chainthru"
                    ))
                    .as_str(),
            );

        assert!(app.inner.transport_node.is_some());
        assert!(app.inner.transport_storage.is_some());
        assert_eq!(app.src_block, BlockId::Number(BlockNumber::Earliest));
        assert_eq!(app.dst_block, BlockId::Number(BlockNumber::Latest));
    }

    #[tokio::test]
    async fn test_transport_wss() {
        let app: App<WebSocket, chainthru_primitives::storage::Postgres> = crate::App::default()
            .with_ws(
                env::var("CHAINTHRU_TEST_WSS_PROVIDER")
                    .unwrap_or(format!("wss://eth.llamarpc.com"))
                    .as_str(),
            )
            .await
            .with_storage(
                env::var("CHAINTHRU_TEST_DATABASE_URL")
                    .unwrap_or(format!(
                        "postgres://postgres:password@localhost:5432/chainthru"
                    ))
                    .as_str(),
            );

        assert!(app.inner.transport_node.is_some());
        assert!(app.inner.transport_storage.is_some());
        assert_eq!(app.src_block, BlockId::Number(BlockNumber::Earliest));
        assert_eq!(app.dst_block, BlockId::Number(BlockNumber::Latest));
    }
}
