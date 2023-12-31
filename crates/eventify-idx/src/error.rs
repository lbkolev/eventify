use ethers_core::types::H256;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    NodeClient(#[from] NodeClientError),

    #[error(transparent)]
    StorageClient(#[from] StorageClientError),

    #[error(transparent)]
    JoinTask(#[from] tokio::task::JoinError),

    #[error(transparent)]
    EventifyPrimitives(#[from] eventify_primitives::Error),

    #[error(transparent)]
    Sql(#[from] sqlx::Error),

    #[error("{0}")]
    InvalidNodeKind(String),

    #[error("{0}")]
    InvalidDatabase(String),
}

#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum NodeClientError {
    #[error("failed to connect to node")]
    Connect,

    #[error("failed to get the latest block number")]
    GetLatestBlock,

    #[error("failed to get block {0}")]
    GetBlock(u64),

    #[error("failed to get transactions from block {0}")]
    GetTransactions(u64),

    #[error("Failed to get logs for criteria {0}")]
    GetLogs(String),
}

#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum StorageClientError {
    #[error("failed to store block {0}")]
    StoreBlock(u64),

    #[error("failed to store transaction {0}")]
    StoreTransaction(H256),

    #[error("failed to store log {0}")]
    StoreLog(H256),

    #[error("failed to store contract {0}")]
    StoreContract(H256),
}
