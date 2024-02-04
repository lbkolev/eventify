use crate::{emit, storage::StoreError, NodeError};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Node(#[from] NodeError),

    #[error(transparent)]
    StoreClient(#[from] StoreError),

    #[error(transparent)]
    JoinTask(#[from] tokio::task::JoinError),

    #[error(transparent)]
    EventifyPrimitives(#[from] eventify_primitives::Error),

    #[error(transparent)]
    EmitError(#[from] emit::EmitError),

    #[error(transparent)]
    Sql(#[from] sqlx::Error),

    #[error("rpc error: {0:?}")]
    RpcError(#[from] eyre::Report),

    #[error("jsonrpsee error: {0:?}")]
    JsonRpsee(#[from] jsonrpsee::core::ClientError),

    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
}
