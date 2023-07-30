use sqlx::postgres::PgPool;
use web3::transports::{ipc::Ipc, ws::WebSocket, Http};
use web3::types::{Block, BlockId, BlockNumber, Transaction};
use web3::{Transport, Web3};

use crate::Result;

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

    pub async fn fetch_block(&self, block: BlockId) -> Result<Option<Block<Transaction>>> {
        let block = self
            .inner
            .transport_node
            .as_ref()
            .expect("Unable to get transport node")
            .eth()
            .block_with_txs(block)
            .await?;

        Ok(block)
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

    pub async fn dbconn(&self) -> Result<PgPool> {
        Ok(self
            .inner
            .transport_db
            .as_ref()
            .expect("Unable to get transport db")
            .clone())
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
