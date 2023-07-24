pub mod app;
pub mod block;
pub mod transaction;

use ethereum_types::{H160, U256};
use web3::transports::{ipc::Ipc, ws::WebSocket, Http};
use web3::types::BlockId;
use web3::{Transport, Web3};

use crate::block::insert_block;
use transaction::erc20::{self, TRANSFER_SIGNATURE};

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
