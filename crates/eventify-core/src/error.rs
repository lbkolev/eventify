#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    JoinTask(#[from] tokio::task::JoinError),

    #[error(transparent)]
    Sql(#[from] sqlx::Error),

    #[error(transparent)]
    JsonRpsee(#[from] jsonrpsee::core::ClientError),

    #[error(transparent)]
    SerdeError(#[from] serde_json::Error),

    #[error("Empty stream")]
    EmptyStream,
}
