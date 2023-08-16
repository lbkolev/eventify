use chainthru_primitives::transaction::IndexedTransaction;
use chainthru_primitives::Insertable;
use ethereum_types::H256;
use sqlx::postgres::PgPool;
use web3::transports::{ipc::Ipc, ws::WebSocket, Http};
use web3::types::{Block, BlockId, BlockNumber, Transaction};
use web3::{Transport, Web3};

use crate::Result;
use chainthru_primitives::{block::IndexedBlock, contract::Contract};

#[derive(Debug)]
pub struct App<T: Transport> {
    inner: Inner<T>,

    pub block_from: BlockId,
    pub block_to: BlockId,
}

impl<T: Transport> Default for App<T> {
    fn default() -> Self {
        Self {
            inner: Inner::default(),
            block_from: BlockId::Number(BlockNumber::Earliest),
            block_to: BlockId::Number(BlockNumber::Latest),
        }
    }
}

impl<T: Transport> App<T> {
    /// Create a new instance of the indexer
    pub fn new(
        transport_node: Option<Web3<T>>,
        transport_db: Option<PgPool>,
        block_from: u64,
        block_to: u64,
    ) -> Self {
        Self {
            inner: Inner::new(transport_node, transport_db),
            block_from: BlockId::Number(block_from.into()),
            block_to: BlockId::Number(block_to.into()),
        }
    }

    pub fn with_from_block(self, block_from: BlockId) -> Self {
        Self { block_from, ..self }
    }

    pub fn with_to_block(self, block_to: BlockId) -> Self {
        Self { block_to, ..self }
    }

    pub fn with_node_conn(self, transport: Web3<T>) -> Self {
        Self {
            inner: Inner::new(Some(transport), self.inner.transport_db),
            ..self
        }
    }

    pub fn with_database_conn(self, database_conn: PgPool) -> Self {
        Self {
            inner: Inner::new(self.inner.transport_node, Some(database_conn)),
            ..self
        }
    }

    pub async fn with_database_url(self, database_url: &str) -> Self {
        Self {
            inner: Inner::new(
                self.inner.transport_node,
                Some(
                    PgPool::connect(database_url)
                        .await
                        .expect("Failed to connect to the database with the provided URL"),
                ),
            ),
            ..self
        }
    }

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

    pub async fn latest_block(&self) -> Result<u64> {
        let block = self
            .inner
            .transport_node
            .as_ref()
            .unwrap()
            .eth()
            .block_number()
            .await?;

        Ok(block.as_u64())
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

            match contract.insert(self.dbconn()).await {
                Ok(_) => log::info!("Contract inserted"),
                Err(e) => log::warn!("Error inserting contract: {:?}", e),
            }
        }

        Ok(())
    }

    pub fn dbconn(&self) -> &PgPool {
        self.inner
            .transport_db
            .as_ref()
            .expect("Unable to get transport db")
    }

    pub async fn src_block(&self) -> u64 {
        match self.block_from {
            BlockId::Number(block) => match block {
                BlockNumber::Number(block) => block.as_u64(),
                _ => 0,
            },
            BlockId::Hash(_) => 0,
        }
    }

    pub async fn dst_block(&self) -> Result<u64> {
        match self.block_to {
            BlockId::Number(block) => match block {
                BlockNumber::Number(block) => Ok(block.as_u64()),
                _ => self.latest_block().await,
            },
            _ => unimplemented!(),
        }
    }
}

impl App<Http> {
    /// Create a new instance of the indexer with the HTTP transport
    #[allow(unused)]
    pub fn with_http(self, node_url: &str) -> Self {
        Self {
            inner: Inner::new(
                Some(Web3::new(
                    Http::new(node_url).expect("Failed to create HTTP transport"),
                )),
                self.inner.transport_db,
            ),
            ..self
        }
    }
}

impl App<Ipc> {
    /// Create a new instance of the indexer with the IPC transport
    #[allow(unused)]
    pub async fn with_ipc(self, node_url: &str) -> Self {
        Self {
            inner: Inner::new(
                Some(Web3::new(
                    Ipc::new(node_url)
                        .await
                        .expect("Failed to create HTTP transport"),
                )),
                self.inner.transport_db,
            ),
            ..self
        }
    }
}

impl App<WebSocket> {
    /// Create a new instance of the indexer with the WebSocket transport
    #[allow(unused)]
    pub async fn with_websocket(self, node_url: &str) -> Self {
        Self {
            inner: Inner::new(
                Some(Web3::new(
                    WebSocket::new(node_url)
                        .await
                        .expect("Failed to create HTTP transport"),
                )),
                self.inner.transport_db,
            ),
            ..self
        }
    }

    /// Create a new instance of the indexer with the WebSocket transport
    ///
    /// An alias for `with_websocket`
    pub async fn with_ws(self, node_url: &str) -> Self {
        self.with_websocket(node_url).await
    }
}

#[derive(Debug)]
struct Inner<T: Transport> {
    transport_node: Option<Web3<T>>,
    transport_db: Option<PgPool>,
}

impl<T: Transport> Default for Inner<T> {
    fn default() -> Self {
        Self {
            transport_node: None,
            transport_db: None,
        }
    }
}

impl<T: Transport> Inner<T> {
    pub fn new(node: Option<Web3<T>>, db: Option<PgPool>) -> Self {
        Self {
            transport_node: node,
            transport_db: db,
        }
    }
}
