use clap::error::ErrorKind;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Invalid transaction input length {0}")]
    InvalidTransactionInputLength(usize),

    #[error("Invalid transaction function signature {0}")]
    InvalidTransactionFunctionSignature(String),

    #[error("Unable to parse criteria file. {0}")]
    InvalidCriteriasFile(String),

    #[error(transparent)]
    Sql(#[from] sqlx::Error),

    #[error(transparent)]
    Migrate(#[from] sqlx::migrate::MigrateError),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Url(#[from] url::ParseError),
}

impl From<Error> for clap::Error {
    fn from(error: Error) -> Self {
        clap::Error::raw(ErrorKind::ValueValidation, error.to_string())
    }
}
