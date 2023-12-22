use clap::error::ErrorKind;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Transaction input length {0}")]
    InvalidTransactionInputLength(usize),

    #[error("Transaction function signature {0}")]
    InvalidTransactionFunctionSignature(String),

    #[error("Unable to parse criteria file. {0}")]
    InvalidCriteriasFile(String),
}

impl From<Error> for clap::Error {
    fn from(error: Error) -> Self {
        clap::Error::raw(ErrorKind::ValueValidation, error.to_string())
    }
}
