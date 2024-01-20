pub mod pg;

use std::fmt::{Debug, Display};

use crate::{error::Error, storage_client, StorageClient};
use eventify_primitives::{Contract, EthBlock, EthLog, EthTransaction};
use sqlx::Pool;

storage_client!(Postgres, Pool<sqlx::postgres::Postgres>);

// Supported storages
#[derive(Debug, Default, Clone, Copy)]
pub enum StorageKind {
    #[default]
    Postgres,
}

impl Display for StorageKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StorageKind::Postgres => write!(f, "postgres"),
        }
    }
}

impl std::str::FromStr for StorageKind {
    type Err = Error;

    fn from_str(s: &str) -> crate::Result<Self> {
        match s.to_lowercase().as_str() {
            "postgres" | "pg" => Ok(StorageKind::Postgres),
            _ => Err(Error::InvalidDatabase(s.to_string())),
        }
    }
}

#[derive(Debug, Clone)]
pub enum StorageClientKind {
    Postgres(Postgres),
}

impl StorageClient for StorageClientKind {
    async fn store_block(&self, block: &EthBlock) -> Result<(), Error> {
        match self {
            StorageClientKind::Postgres(inner) => inner.store_block(block).await,
        }
    }

    async fn store_transaction(&self, transaction: &EthTransaction) -> Result<(), Error> {
        match self {
            StorageClientKind::Postgres(inner) => inner.store_transaction(transaction).await,
        }
    }

    async fn store_log(&self, log: &EthLog) -> Result<(), Error> {
        match self {
            StorageClientKind::Postgres(inner) => inner.store_log(log).await,
        }
    }

    async fn store_contract(&self, contract: &Contract) -> Result<(), Error> {
        match self {
            StorageClientKind::Postgres(inner) => inner.store_contract(contract).await,
        }
    }
}
