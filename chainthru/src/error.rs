#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Invalid Node URL scheme {0}")]
    NodeURLScheme(String),

    #[error(transparent)]
    NodeURLParser(#[from] url::ParseError),

    #[error(transparent)]
    MigrateError(#[from] sqlx::migrate::MigrateError),

    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),

    #[error(transparent)]
    IndexerError(#[from] chainthru_index::Error),

    #[error(transparent)]
    ServerError(#[from] chainthru_server::Error),
}
