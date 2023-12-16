#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Invalid Node URL scheme {0}")]
    NodeURLScheme(String),

    #[error(transparent)]
    IndexerError(#[from] eventify_idx::Error),

    #[error(transparent)]
    ServerError(#[from] eventify_http_server::Error),

    #[error(transparent)]
    TypesError(#[from] eventify_primitives::Error),

    #[error(transparent)]
    NodeURLParser(#[from] url::ParseError),

    #[error(transparent)]
    MigrateError(#[from] sqlx::migrate::MigrateError),

    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),
}
