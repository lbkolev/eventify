use crate::{store::StoreError, NodeProviderError};
use alloy_primitives::B256;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    NodeProvider(#[from] NodeProviderError),

    #[error(transparent)]
    StoreClient(#[from] StoreError),

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

    #[error("jsonrpsee error: {0:?}")]
    JsonRpsee(#[from] jsonrpsee::core::ClientError),
}
