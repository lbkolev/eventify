use async_trait::async_trait;
use ethereum_types::U64;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, Row};
use web3::types::{H160, H256, H64, U256};

use crate::{Insert, Result};

/// Minimum block representation
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
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
            number: row.try_get("number").ok().map(|v: i64| v.into()),
            gas_used: row.try_get("gas_used").ok().map(U256::from_big_endian),
            gas_limit: row.try_get("gas_limit").ok().map(U256::from_big_endian),
            base_fee_per_gas: row
                .try_get("base_fee_per_gas")
                .ok()
                .map(U256::from_big_endian),
            difficulty: row.try_get("difficulty").ok().map(U256::from_big_endian),
            total_difficulty: row
                .try_get("total_difficulty")
                .ok()
                .map(U256::from_big_endian),
            transactions: row
                .try_get("transactions")
                .ok()
                .map(|v: i32| v as u32),
            size: row.try_get("size").ok().map(U256::from_big_endian),
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
        self.total_difficulty
            .map(|v| v.to_big_endian(&mut total_difficulty_slice));

        let mut size_slice = [0u8; 32];
        self.size.map(|v| v.to_big_endian(&mut size_slice));

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
