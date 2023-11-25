#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Invalid transaction input length {0}")]
    InvalidTransactionInputLength(usize),

    #[error("Invalid transaction function signature {0}")]
    InvalidTransactionFunctionSignature(String),

    #[error(transparent)]
    Sql(#[from] sqlx::Error),

    #[error(transparent)]
    Migrate(#[from] sqlx::migrate::MigrateError),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Url(#[from] url::ParseError),
}
