pub mod pg;

use std::fmt::{Debug, Display};

use crate::error::{Error, StorageClientError};
use eventify_primitives::{Block, Contract, Log, Transaction};

#[async_trait::async_trait]
pub trait StorageClient: 'static + Clone + Debug + Send + Sync {
    async fn store_block(&self, block: &Block) -> Result<(), StorageClientError>;
    async fn store_transaction(&self, transaction: &Transaction) -> Result<(), StorageClientError>;
    async fn store_log(&self, log: &Log) -> Result<(), StorageClientError>;
    async fn store_contract(&self, contract: &Contract) -> Result<(), StorageClientError>;
}

#[async_trait::async_trait]
pub trait Auth {
    async fn connect(url: &str) -> Self;
}

#[cfg(feature = "postgres")]
pub mod postgres_client {
    use crate::storage_client;
    use sqlx::Pool;

    storage_client!(Postgres, Pool<sqlx::postgres::Postgres>);
}

pub use postgres_client::Postgres;

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

#[async_trait::async_trait]
impl StorageClient for StorageClientKind {
    async fn store_block(&self, block: &Block) -> Result<(), StorageClientError> {
        match self {
            StorageClientKind::Postgres(inner) => inner.store_block(block).await,
        }
    }

    async fn store_transaction(&self, transaction: &Transaction) -> Result<(), StorageClientError> {
        match self {
            StorageClientKind::Postgres(inner) => inner.store_transaction(transaction).await,
        }
    }

    async fn store_log(&self, log: &Log) -> Result<(), StorageClientError> {
        match self {
            StorageClientKind::Postgres(inner) => inner.store_log(log).await,
        }
    }

    async fn store_contract(&self, contract: &Contract) -> Result<(), StorageClientError> {
        match self {
            StorageClientKind::Postgres(inner) => inner.store_contract(contract).await,
        }
    }
}
