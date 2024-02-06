//#![allow(clippy::option_map_unit_fn)]

pub mod eth;

use alloy_primitives::{Address, B256};
use eventify_configs::database::DatabaseConfig;
use sqlx::{pool::PoolOptions, postgres::Postgres, Pool};

#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum StorageError {
    #[error("Failed to store block {hash}. {err}")]
    StoreBlockFailed { hash: B256, err: String },

    #[error("Failed to store transaction {hash}. {err}")]
    StoreTransactionFailed { hash: B256, err: String },

    #[error("Failed to store log from addr {addr}. {err}")]
    StoreLogFailed { addr: Address, err: String },

    #[error("Failed to store contract from tx {hash}. {err}")]
    StoreContractFailed { hash: B256, err: String },
}

#[derive(Clone, Debug)]
pub struct Storage {
    pub inner: Pool<Postgres>,
    pub config: DatabaseConfig,
}

impl std::ops::Deref for Storage {
    type Target = Pool<Postgres>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl std::ops::DerefMut for Storage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl From<Pool<Postgres>> for Storage {
    fn from(inner: Pool<Postgres>) -> Self {
        Self {
            inner,
            config: DatabaseConfig::default(),
        }
    }
}

impl std::fmt::Display for Storage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", stringify!(Storage))
    }
}

impl Storage {
    pub async fn new(pool: Pool<Postgres>, config: DatabaseConfig) -> Self {
        Self {
            inner: pool,
            config,
        }
    }

    pub async fn connect(config: DatabaseConfig) -> Self {
        Self {
            inner: PoolOptions::new()
                .acquire_timeout(std::time::Duration::from_secs(5))
                .connect_lazy_with(config.with_db()),
            config,
        }
    }

    pub fn inner(&self) -> &Pool<Postgres> {
        &self.inner
    }

    pub fn with_inner(mut self, inner: Pool<Postgres>) -> Self {
        self.inner = inner;
        self
    }

    pub fn with_config(mut self, config: DatabaseConfig) -> Self {
        self.config = config;
        self
    }
}
