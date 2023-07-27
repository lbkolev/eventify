pub mod erc20;

use async_trait::async_trait;
use ethereum_types::{H160, H256, U256};

#[derive(Debug)]
pub enum TransactionType {
    ERC20,
    ERC721,
    ERC1155,
    Other,
}

#[async_trait]
pub trait DBInsert {
    async fn insert(&self, contract: H160, db_conn: &sqlx::PgPool) -> Result<(), sqlx::Error>;

    async fn insert_where(
        &self,
        contract: H160,
        db_conn: &sqlx::PgPool,
        where_clause: &str,
    ) -> Result<(), sqlx::Error>;
}

struct Transaction {
    inner: web3::types::Transaction,
}

impl Transaction {
    pub fn new(inner: web3::types::Transaction) -> Self {
        Self { inner }
    }

    pub fn hash(&self) -> H256 {
        self.inner.hash
    }

    pub fn from(&self) -> Option<H160> {
        self.inner.from
    }

    pub fn to(&self) -> Option<H160> {
        self.inner.to
    }

    pub fn value(&self) -> U256 {
        self.inner.value
    }

    pub fn gas(&self) -> U256 {
        self.inner.gas
    }

    pub fn gas_price(&self) -> Option<U256> {
        self.inner.gas_price
    }

    pub fn nonce(&self) -> U256 {
        self.inner.nonce
    }

    pub fn input(&self) -> Vec<u8> {
        self.inner.input.0
    }

    pub fn transaction_type(&self) -> TransactionType {
        if self.input().starts_with(TRANSFER_SIGNATURE) && self.input().len() == 68 {
            TransactionType::ERC20
        } else {
            TransactionType::Other
        }
    }

    pub async fn insert(&self, db_conn: &sqlx::PgPool) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO transaction
            (hash, from, to, value, gas, gas_price, nonce, input, transaction_type)
            VALUES
            ($1, $2, $3, $4::numeric, $5::numeric, $6::numeric, $7::numeric, $8, $9)",
        )
        .bind(self.hash().as_bytes())
        .bind(self.from().as_bytes())
        .bind(self.to().as_bytes())
        .bind(self.value().to_string())
        .bind(self.gas().to_string())
        .bind(self.gas_price().to_string())
        .bind(self.nonce().to_string())
        .bind(self.input())
        .bind(self.transaction_type().to_string())
        .execute(db_conn)
        .await?;

        Ok(())
    }
}
