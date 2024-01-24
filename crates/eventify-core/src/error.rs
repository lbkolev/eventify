use std::num::ParseIntError;

use crate::NodeProviderError;
use alloy_primitives::B256;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    NodeProvider(#[from] NodeProviderError),

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

    #[error("rpc error: {0:?}")]
    RpcError(#[from] eyre::Report),
}

#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum StorageClientError {
    #[error("Failed to store block {0}")]
    StoreBlockFailed(u64),

    #[error("Failed to store transaction {0}")]
    StoreTransactionFailed(B256),

    #[error("Failed to store log {0}")]
    StoreLogFailed(B256),

    #[error("Failed to store contract {0}")]
    StoreContractFailed(B256),
}
