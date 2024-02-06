use crate::{storage::StorageError, NetworkError};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Node(#[from] NetworkError),

    #[error(transparent)]
    StoreClient(#[from] StorageError),

    #[error(transparent)]
    JoinTask(#[from] tokio::task::JoinError),

    #[error(transparent)]
    Sql(#[from] sqlx::Error),

    #[error("rpc error: {0:?}")]
    RpcError(#[from] eyre::Report),

    #[error(transparent)]
    JsonRpsee(#[from] reconnecting_jsonrpsee_ws_client::DisconnectWillReconnect),

    #[error(transparent)]
    SerdeError(#[from] serde_json::Error),
}
