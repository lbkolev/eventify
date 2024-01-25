use clap::error::ErrorKind;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("transaction input length {0}")]
    InvalidTransactionInputLength(usize),

    #[error("transaction function signature {0}")]
    InvalidTransactionFunctionSignature(String),

    #[error("unable to parse criteria file. {0}")]
    InvalidCriteriaFile(String),
}

impl From<Error> for clap::Error {
    fn from(error: Error) -> Self {
        clap::Error::raw(ErrorKind::ValueValidation, error.to_string())
    }
}
