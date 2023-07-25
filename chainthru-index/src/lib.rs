pub mod app;
pub mod block;
pub mod transaction;

use web3::types::Transaction;

use crate::transaction::{erc20::TRANSFER_SIGNATURE, TransactionType};

type Result<T> = std::result::Result<T, crate::Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("SQL error: {0}")]
    Sql(#[from] sqlx::Error),

    #[error("Web3 error: {0}")]
    Web3(#[from] web3::Error),

    #[error("Migration error: {0}")]
    Migrate(#[from] sqlx::migrate::MigrateError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("URL error: {0}")]
    Url(#[from] url::ParseError),
}

pub fn transaction_type(transaction: Transaction) -> transaction::TransactionType {
    if transaction.input.0.starts_with(TRANSFER_SIGNATURE) && transaction.input.0.len() == 68 {
        TransactionType::ERC20
    } else {
        TransactionType::Other
    }
}
