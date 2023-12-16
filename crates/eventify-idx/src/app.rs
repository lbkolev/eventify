use std::sync::Arc;

use alloy_primitives::BlockNumber;
use eventify_primitives::{Criterias, IndexedTransaction};

use ethers_core::types::{Block, BlockId, Filter, Log, Transaction, TxHash, H256};
use ethers_providers::{
    Http, Ipc, JsonRpcClient, Middleware, Provider as NodeProvider, SubscriptionStream, Ws,
};

use crate::Result;
use eventify_primitives::{
    block::IndexedBlock,
    storage::{Auth, Storage},
};

/// The `App` struct represents an application with a JSON-RPC client (`T`) and storage (`U`).
/// It manages the interaction between the blockchain and storage, keeping track of the source
/// and destination block numbers for operations.
///
/// # Type Parameters
/// - `T`: A JSON-RPC client that implements `JsonRpcClient`, `Clone`, `Send`, and `Sync`.
/// - `U`: A storage system that implements `Storage`, `Auth`, `Clone`, `Send`, and `Sync`.
#[derive(Clone, Debug)]
pub struct App<T, U>
where
    T: JsonRpcClient + Clone + Send + Sync,
    U: Storage + Auth + Clone + Send + Sync,
{
    inner: Providers<T, U>,

    /// The starting block number from which the `App` operates.
    pub src_block: BlockNumber,

    /// The ending block number up to which the `App` operates.
    pub dst_block: BlockNumber,
}

impl<T, U> Default for App<T, U>
where
    T: JsonRpcClient + Clone + Send + Sync,
    U: Storage + Auth + Clone + Send + Sync,
{
    /// Creates a new `App` instance with default values.
    ///
    /// The `src_block` is set to `0`, indicating the start of the blockchain,
    /// and `dst_block` is set to `BlockNumber::MAX`, representing the end of the blockchain.
    fn default() -> Self {
        Self {
            inner: Providers::default(),
            src_block: 0,
            dst_block: BlockNumber::MAX,
        }
    }
}

impl<T, U> App<T, U>
where
    T: JsonRpcClient + Clone + Send + Sync,
    U: Storage + Auth + Clone + Send + Sync,
{
    /// Create a new instance of the indexer
    pub fn new(
        transport_node: Option<NodeProvider<T>>,
        transport_storage: Option<U>,
        src_block: BlockNumber,
        dst_block: BlockNumber,
    ) -> Self {
        Self {
            inner: Providers::new(
                transport_node.map(Arc::new),
                transport_storage.map(Arc::new),
            ),
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
            inner: Providers::new(Some(Arc::new(transport)), self.inner.transport_storage),
            ..self
        }
    }

    pub fn with_storage(self, url: &str) -> Self {
        Self {
            inner: Providers::new(
                self.inner.transport_node,
                Some(Arc::new(U::connect_lazy(url))),
            ),
            ..self
        }
    }

    pub fn src_block(&self) -> u64 {
        self.src_block
    }

    pub fn dst_block(&self) -> u64 {
        self.dst_block
    }

    pub fn storage_conn(&self) -> Result<&U> {
        self.inner
            .transport_storage
            .as_ref()
            .map(|arc_storage| arc_storage.as_ref())
            .ok_or(crate::Error::MissingTransportStorage)
    }

    /// Retrieves block details along with full transaction objects for a given block ID.
    ///
    /// # Arguments
    /// * `block` - The block ID for which to fetch details.
    ///
    /// # Returns
    /// Returns a `Result` containing the block details on success, or an error if the block
    /// cannot be fetched or if the transport node is unavailable.
    pub async fn fetch_block_with_txs(&self, block: BlockId) -> Result<Block<Transaction>> {
        let transport_node = self
            .inner
            .transport_node
            .as_ref()
            .ok_or(crate::Error::MissingTransportNode)?;

        let block_result = transport_node
            .get_block_with_txs(block)
            .await
            .map_err(|e| crate::Error::FetchBlock(format!("{}", e)))?
            .ok_or(crate::Error::FetchBlock("Block not found".to_string()))?;

        Ok(block_result)
    }

    /// Retrieves block details for a given block ID.
    ///
    /// This function does not return transaction objects.
    pub async fn fetch_block(&self, block: BlockId) -> Result<Block<TxHash>> {
        let transport_node = self
            .inner
            .transport_node
            .as_ref()
            .ok_or(crate::Error::MissingTransportNode)?;

        let block_result = transport_node
            .get_block(block)
            .await
            .map_err(|e| crate::Error::FetchBlock(format!("{}", e)))?
            .ok_or(crate::Error::FetchBlock("Block not found".to_string()))?;

        Ok(block_result)
    }

    /// Fetches logs based on the specified filter.
    ///
    /// # Arguments
    /// * `criteria` - The filter criteria used to fetch the logs.
    ///
    /// # Returns
    /// Returns a `Result` containing a vector of logs on success, or an error if the logs
    /// cannot be fetched or if the transport node is unavailable.
    // TODO:
    // Improve the exposed API; it doesn't make much sense to require a block to fetch logs from,
    // when we've already defined src & dst blocks in the type itself
    pub async fn fetch_logs(
        &self,
        criterias: &Criterias,
        block: BlockNumber,
    ) -> Result<Vec<ethers_core::types::Log>> {
        let transport_node = self
            .inner
            .transport_node
            .as_ref()
            .ok_or(crate::Error::MissingTransportNode)?;

        let mut resp = vec![];
        for criterias in criterias.0.iter() {
            log::info!("Fetching logs for criteria: {}", criterias.name());
            let ir: Filter = criterias.into();
            let filter: Filter = ir.from_block(block).to_block(block);

            resp.extend(
                transport_node
                    .get_logs(&filter)
                    .await
                    .map_err(|e| crate::Error::FetchLog(format!("Failed to fetch logs: {}", e)))?,
            );
        }

        Ok(resp)
    }

    /// Fetches transactions for a specified block number.
    pub async fn fetch_transactions(&self, block: BlockNumber) -> Result<Vec<Transaction>> {
        let transport_node = self
            .inner
            .transport_node
            .as_ref()
            .ok_or(crate::Error::MissingTransportNode)?;

        let block = transport_node
            .get_block_with_txs(BlockId::Number(ethers_core::types::BlockNumber::Number(
                block.into(),
            )))
            .await
            .map_err(|e| crate::Error::FetchBlock(format!("{}", e)))?
            .ok_or(crate::Error::FetchBlock("Block not found".to_string()))?;

        Ok(block.transactions)
    }

    /// Fetches indexed block and transaction objects for a specified block number.
    ///
    /// # Arguments
    /// * `block` - The block number for which to fetch the data.
    ///
    /// # Returns
    /// Returns a `Result` containing the indexed block and its transactions on success,
    /// or an error if the block cannot be fetched.
    pub async fn fetch_indexed_data(
        &self,
        block: BlockNumber,
    ) -> Result<(IndexedBlock, Vec<IndexedTransaction>)> {
        let fetched_block = self
            .fetch_block_with_txs(BlockId::Number(ethers_core::types::BlockNumber::Number(
                block.into(),
            )))
            .await?;

        // Clone hash and number before moving transactions
        let block_hash = fetched_block.hash.unwrap_or(H256::zero());
        let block_number = fetched_block.number.unwrap_or_default();

        let transactions = fetched_block
            .transactions
            .clone()
            .into_iter()
            .map(IndexedTransaction::from)
            .collect();

        log::info!("Fetched block {} with hash {:?}", block_number, block_hash);

        Ok((IndexedBlock::from(fetched_block), transactions))
    }

    /// Returns the latest finalized block number.
    ///
    /// This function queries the underlying transport node for the current block number.
    /// If the transport node is not set, or if there is an error in fetching the block number,
    /// the function will return an appropriate error.
    pub async fn get_latest_block(&self) -> Result<u64> {
        let transport_node = self
            .inner
            .transport_node
            .as_ref()
            .ok_or_else(|| crate::Error::MissingTransportNode)?;

        let block_number = transport_node
            .get_block_number()
            .await
            .map_err(|e| crate::Error::FetchBlockNumberError(e.to_string()))?;

        Ok(block_number.as_u64())
    }

    /// Checks whether the provided block number is the latest finalized block
    pub async fn is_latest_block(&self, block: u64) -> Result<bool> {
        Ok(self.get_latest_block().await? == block)
    }
}

impl<U: Storage + Auth + Clone + Send + Sync> App<Http, U> {
    /// Configures the application to use an HTTP transport node.
    ///
    /// # Example
    /// ```
    /// use ethers_providers::Http;
    /// use eventify_idx::App;
    /// use eventify_primitives::storage::Postgres;
    /// # async fn run() -> Result<(), eventify_idx::error::Error> {
    /// let app: App<Http, Postgres> = App::default().with_http("http://localhost:8545")?;
    /// // Use `app` for further operations...
    ///     # Ok(())
    /// # }
    /// ```
    ///
    /// # Arguments
    /// * `node_url` - The URL of the HTTP node.
    ///
    /// # Errors
    /// Returns an error if the URL parsing fails or the HTTP transport cannot be created.
    pub fn with_http(self, node_url: &str) -> Result<Self> {
        let parsed_url = url::Url::parse(node_url)
            .map_err(|e| crate::Error::UrlParseError(node_url.to_string(), e.to_string()))?;

        let http_transport = NodeProvider::new(Http::new(parsed_url));

        Ok(Self {
            inner: Providers::new(Some(Arc::new(http_transport)), self.inner.transport_storage),
            ..self
        })
    }
}

impl<U: Storage + Auth + Clone + Send + Sync> App<Ipc, U> {
    /// Creates a new instance of the App with the IPC transport.
    ///
    /// # Example
    /// ```
    /// use ethers_providers::Ipc;
    /// use eventify_idx::App;
    /// use eventify_primitives::storage::Postgres;
    /// # async fn run() -> Result<(), eventify_idx::error::Error> {
    /// let app: App<Ipc, Postgres> = App::default().with_ipc("ipc://path/to/socket").await?;
    /// // use app...
    ///     # Ok(())
    /// # }
    /// ```
    ///
    /// # Arguments
    /// * `node_url` - The URL for the IPC node.
    ///
    /// # Errors
    /// Returns an error if the IPC transport creation fails.
    pub async fn with_ipc(self, node_url: &str) -> Result<Self> {
        let ipc_transport = Ipc::connect(node_url).await.map_err(|e| {
            crate::Error::IpcTransportCreationError(node_url.to_string(), e.to_string())
        })?;

        Ok(Self {
            inner: Providers::new(
                Some(Arc::new(NodeProvider::new(ipc_transport))),
                self.inner.transport_storage,
            ),
            ..self
        })
    }

    pub async fn subscribe_blocks(&self) -> Result<SubscriptionStream<Ipc, Block<TxHash>>> {
        let transport_node = self
            .inner
            .transport_node
            .as_ref()
            .ok_or_else(|| crate::Error::MissingTransportNode)?;

        transport_node
            .subscribe_blocks()
            .await
            .map_err(|e| crate::Error::SubscriptionNewBlock(e.to_string()))
    }

    pub async fn subscribe_logs(&self, filter: Filter) -> Result<SubscriptionStream<Ipc, Log>> {
        let transport_node = self
            .inner
            .transport_node
            .as_ref()
            .ok_or_else(|| crate::Error::MissingTransportNode)?;

        transport_node
            .subscribe_logs(&filter)
            .await
            .map_err(|e| crate::Error::SubscriptionNewBlock(e.to_string()))
    }
}

impl<U: Storage + Auth + Clone + Send + Sync> App<Ws, U> {
    /// Creates a new instance of the App with the WebSocket transport.
    ///
    /// # Example
    /// ```
    /// use ethers_providers::Ws;
    /// use eventify_idx::App;
    /// use eventify_primitives::storage::Postgres;
    /// # async fn run() -> Result<(), eventify_idx::error::Error> {
    /// let app: App<Ws, Postgres> = App::default().with_websocket("ws://localhost:8546").await?;
    /// // Use `app` for further operations...
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Arguments
    /// * `node_url` - The URL for the WebSocket node.
    ///
    /// # Errors
    /// Returns an error if the WebSocket transport creation fails.
    pub async fn with_websocket(self, node_url: &str) -> Result<Self> {
        let ws_transport = Ws::connect(node_url).await.map_err(|e| {
            crate::Error::WsTransportCreationError(node_url.to_string(), e.to_string())
        })?;

        Ok(Self {
            inner: Providers::new(
                Some(Arc::new(NodeProvider::new(ws_transport))),
                self.inner.transport_storage,
            ),
            ..self
        })
    }

    /// Creates a new instance of the App with the WebSocket transport
    ///
    /// An alias for [`with_websocket`]
    pub async fn with_ws(self, node_url: &str) -> Result<Self> {
        self.with_websocket(node_url).await
    }

    pub async fn subscribe_blocks(&self) -> Result<SubscriptionStream<Ws, Block<TxHash>>> {
        let transport_node = self
            .inner
            .transport_node
            .as_ref()
            .ok_or_else(|| crate::Error::MissingTransportNode)?;

        transport_node
            .subscribe_blocks()
            .await
            .map_err(|e| crate::Error::SubscriptionNewBlock(e.to_string()))
    }

    pub async fn subscribe_logs(&self, filter: Filter) -> Result<SubscriptionStream<Ws, Log>> {
        let transport_node = self
            .inner
            .transport_node
            .as_ref()
            .ok_or_else(|| crate::Error::MissingTransportNode)?;

        transport_node
            .subscribe_logs(&filter)
            .await
            .map_err(|e| crate::Error::SubscriptionNewLog(e.to_string()))
    }
}

#[derive(Clone, Debug)]
struct Providers<T, U>
where
    T: JsonRpcClient + Clone + Send + Sync,
    U: Storage + Auth + Clone + Send + Sync,
{
    transport_node: Option<Arc<NodeProvider<T>>>,
    transport_storage: Option<Arc<U>>,
}

impl<T, U> Default for Providers<T, U>
where
    T: JsonRpcClient + Clone + Send + Sync,
    U: Storage + Auth + Clone + Send + Sync,
{
    fn default() -> Self {
        Self {
            transport_node: None,
            transport_storage: None,
        }
    }
}

impl<T, U> Providers<T, U>
where
    T: JsonRpcClient + Clone + Send + Sync,
    U: Storage + Auth + Clone + Send + Sync,
{
    pub fn new(node: Option<Arc<NodeProvider<T>>>, db: Option<Arc<U>>) -> Self {
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
        let app: App<Http, eventify_primitives::storage::Postgres> = crate::App::default();

        assert!(app.inner.transport_node.is_none());
        assert!(app.inner.transport_storage.is_none());
        assert_eq!(app.src_block, 0);
        assert_eq!(app.dst_block, BlockNumber::MAX);
    }

    #[tokio::test]
    async fn test_transport_https() {
        let app: App<Http, eventify_primitives::storage::Postgres> = crate::App::default()
            .with_storage(
                env::var("eventify_TEST_DATABASE_URL")
                    .unwrap_or("postgres://postgres:password@localhost:5432/eventify".to_string())
                    .as_str(),
            )
            .with_http(
                env::var("eventify_TEST_HTTPS_PROVIDER")
                    .unwrap_or("https://eth.llamarpc.com".to_string())
                    .as_str(),
            )
            .unwrap();

        assert!(app.inner.transport_node.is_some());
        assert!(app.inner.transport_storage.is_some());
        assert_eq!(app.src_block, 0);
        assert_eq!(app.dst_block, BlockNumber::MAX);
    }

    #[tokio::test]
    async fn test_transport_wss() {
        let app: App<Ws, eventify_primitives::storage::Postgres> = crate::App::default()
            .with_storage(
                env::var("eventify_TEST_DATABASE_URL")
                    .unwrap_or("postgres://postgres:password@localhost:5432/eventify".to_string())
                    .as_str(),
            )
            .with_ws(
                env::var("eventify_TEST_WSS_PROVIDER")
                    .unwrap_or("wss://eth.llamarpc.com".to_string())
                    .as_str(),
            )
            .await
            .unwrap();

        assert!(app.inner.transport_node.is_some());
        assert!(app.inner.transport_storage.is_some());
        assert_eq!(app.src_block, 0);
        assert_eq!(app.dst_block, BlockNumber::MAX);
    }
}
