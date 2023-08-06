//! chainthru-types contains all the types used in the chainthru project.

use async_trait::async_trait;
use core::str::FromStr;
use ethereum_types::U64;
use serde_aux::field_attributes::deserialize_number_from_string;
use sqlx::postgres::{PgConnectOptions, PgSslMode};
use sqlx::{FromRow, PgPool, Row};
use url::Url;
use web3::types::{H160, H256, H64, U256};

pub mod erc20;
pub mod erc721;
pub mod macros;

#[derive(serde::Deserialize, Clone)]
pub struct DatabaseSettings {
    pub database_name: String,
    pub host: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub username: String,
    pub password: String,
    pub require_ssl: bool,
}

impl DatabaseSettings {
    pub fn without_db(&self) -> PgConnectOptions {
        let require_ssl = if self.require_ssl {
            PgSslMode::Require
        } else {
            PgSslMode::Prefer
        };

        PgConnectOptions::new()
            .host(&self.host)
            .username(&self.username)
            .password(&self.password)
            .port(self.port)
            .ssl_mode(require_ssl)
    }

    pub fn with_db(&self) -> PgConnectOptions {
        self.without_db().database(&self.database_name)
    }
}

impl From<String> for DatabaseSettings {
    fn from(s: String) -> Self {
        let url = Url::parse(&s).expect("Invalid database URL");

        Self {
            database_name: url.path().trim_start_matches('/').to_owned(),
            host: url.host_str().unwrap_or("localhost").to_owned(),
            port: url.port().unwrap_or(5432),
            username: url.username().to_owned(),
            password: url.password().unwrap_or("").to_owned(),
            require_ssl: false,
        }
    }
}

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

type Result<T> = std::result::Result<T, Error>;

#[async_trait::async_trait]
pub trait Insert: Sized {
    async fn insert(&self, conn: &PgPool) -> Result<()>;
}

/// Minimum block representation
#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct IndexedBlock {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash: Option<H256>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_hash: Option<H256>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub uncles_hash: Option<H256>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<H160>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_root: Option<H256>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub transactions_root: Option<H256>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipts_root: Option<H256>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<U64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_used: Option<U256>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_limit: Option<U256>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_fee_per_gas: Option<U256>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub difficulty: Option<U256>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_difficulty: Option<U256>,

    /// The number of transactions present in the block
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transactions: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<U256>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<H64>,
}

impl FromRow<'_, sqlx::postgres::PgRow> for IndexedBlock {
    fn from_row(row: &sqlx::postgres::PgRow) -> std::result::Result<Self, sqlx::Error> {
        Ok(IndexedBlock {
            hash: row.try_get("hash").ok().map(H256::from_slice),
            parent_hash: row.try_get("parent_hash").ok().map(H256::from_slice),
            uncles_hash: row.try_get("uncles_hash").ok().map(H256::from_slice),
            author: row.try_get("author").ok().map(H160::from_slice),
            state_root: row.try_get("state_root").ok().map(H256::from_slice),
            transactions_root: row.try_get("transactions_root").ok().map(H256::from_slice),
            receipts_root: row.try_get("receipts_root").ok().map(H256::from_slice),
            number: row
                .try_get::<&str, &str>("number")
                .ok()
                .map(|v| U64::from_str(v).expect("Invalid block number")),
            gas_used: row
                .try_get::<&str, &str>("gas_used")
                .ok()
                .map(|v| U256::from_str(v).expect("Invalid gas_used")),
            gas_limit: row
                .try_get::<&str, &str>("gas_limit")
                .ok()
                .map(|v| U256::from_str(v).expect("Invalid gas_limit")),
            base_fee_per_gas: row
                .try_get::<&str, &str>("base_fee_per_gas")
                .ok()
                .map(|v| U256::from_str(v).expect("Invalid base_fee_per_gas")),
            difficulty: row
                .try_get::<&str, &str>("difficulty")
                .ok()
                .map(|v| U256::from_str(v).expect("Invalid difficulty")),
            total_difficulty: row
                .try_get::<&str, &str>("total_difficulty")
                .ok()
                .map(|v| U256::from_str(v).expect("Invalid total_difficulty")),
            transactions: row.try_get("transactions").ok().map(|v: i64| v as u32),
            size: row
                .try_get::<&str, &str>("size")
                .ok()
                .map(|v| U256::from_str(v).expect("Invalid size")),
            nonce: row.try_get("nonce").ok().map(H64::from_slice),
        })
    }
}

impl From<web3::types::Block<web3::types::Transaction>> for IndexedBlock {
    fn from(block: web3::types::Block<web3::types::Transaction>) -> Self {
        IndexedBlock {
            hash: block.hash,
            parent_hash: Some(block.parent_hash),
            uncles_hash: Some(block.uncles_hash),
            author: Some(block.author),
            state_root: Some(block.state_root),
            transactions_root: Some(block.transactions_root),
            receipts_root: Some(block.receipts_root),
            number: block.number,
            gas_used: Some(block.gas_used),
            gas_limit: Some(block.gas_limit),
            base_fee_per_gas: block.base_fee_per_gas,
            difficulty: Some(block.difficulty),
            total_difficulty: block.total_difficulty,
            transactions: Some(block.transactions.len() as u32),
            size: block.size,
            nonce: block.nonce,
        }
    }
}

#[async_trait]
impl Insert for IndexedBlock {
    async fn insert(&self, conn: &PgPool) -> Result<()> {
        let mut number_slice = [0u8; 8];
        self.number.map(|v| v.to_big_endian(&mut number_slice));

        let mut gas_slice = [0u8; 32];
        self.gas_used.map(|v| v.to_big_endian(&mut gas_slice));

        let mut gas_limit_slice = [0u8; 32];
        self.gas_limit
            .map(|v| v.to_big_endian(&mut gas_limit_slice));

        let mut base_fee_per_gas_slice = [0u8; 32];
        self.base_fee_per_gas
            .map(|v| v.to_big_endian(&mut base_fee_per_gas_slice));

        let mut difficulty_slice = [0u8; 32];
        self.difficulty
            .map(|v| v.to_big_endian(&mut difficulty_slice));

        let mut total_difficulty_slice = [0u8; 32];
        if let Some(total_difficulty) = self.total_difficulty {
            total_difficulty.to_big_endian(&mut total_difficulty_slice);
        }

        let mut size_slice = [0u8; 32];
        if let Some(size) = self.size {
            size.to_big_endian(&mut size_slice);
        }

        sqlx::query(
            "INSERT INTO block
            (hash, parent_hash, uncles_hash, author, state_root, transactions_root, receipts_root, number, gas_used, gas_limit, base_fee_per_gas, difficulty, total_difficulty, transactions, size, nonce)
            VALUES
            ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)
            ON CONFLICT DO NOTHING
            ",
        )
            .bind(self.hash.unwrap_or(H256::zero()).as_bytes())
            .bind(self.parent_hash.map(|v| v.as_bytes().to_owned()))
            .bind(self.uncles_hash.map(|v| v.as_bytes().to_owned()))
            .bind(self.author.map(|v| v.as_bytes().to_owned()))
            .bind(self.state_root.map(|v| v.as_bytes().to_owned()))
            .bind(self.transactions_root.map(|v| v.as_bytes().to_owned()))
            .bind(self.receipts_root.map(|v| v.as_bytes().to_owned()))
            .bind(self.number.unwrap_or(U64::zero()).as_u64() as i32)
            .bind(gas_slice)
            .bind(gas_limit_slice)
            .bind(base_fee_per_gas_slice)
            .bind(difficulty_slice)
            .bind(total_difficulty_slice)
            .bind(self.transactions.unwrap_or(0) as i32)
            .bind(size_slice)
            .bind(self.nonce.unwrap_or(H64::zero()).as_bytes())
            .execute(conn).await?;

        Ok(())
    }
}
