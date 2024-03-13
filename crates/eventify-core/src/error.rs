#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    RpcError(#[from] reconnecting_jsonrpsee_ws_client::RpcError),

    #[error(transparent)]
    RpcReconnect(#[from] reconnecting_jsonrpsee_ws_client::DisconnectWillReconnect),

    #[error(transparent)]
    JoinTask(#[from] tokio::task::JoinError),

    #[error(transparent)]
    Sql(#[from] sqlx::Error),

    #[error(transparent)]
    SerdeError(#[from] serde_json::Error),

    #[error("Empty stream")]
    EmptyStream,

    #[error(transparent)]
    Eyre(#[from] eyre::Error),

    #[error(transparent)]
    SignalRecv(#[from] tokio::sync::watch::error::RecvError),
}
