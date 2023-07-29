pub mod erc20;

use async_trait::async_trait;
use ethereum_types::{H160, H256, U256};

use crate::transaction::erc20::{TRANSFER_SIGNATURE};
use crate::Result;

#[derive(Debug)]
pub enum TransactionType {
    ERC20,
    ERC721,
    ERC1155,
    Other,
}

#[async_trait]
pub trait Process {
    /// Process the transaction and insert everything relevant into the storage.
    async fn process(&self);
}

#[async_trait]
pub trait DBInsert {
    async fn insert(&self, db_conn: &sqlx::PgPool) -> Result<()>;

    async fn insert_where(&self, db_conn: &sqlx::PgPool, where_clause: &str) -> Result<()>;
}

pub struct Transaction {
    pub inner: web3::types::Transaction,

    pub r#type: TransactionType,
}

impl Transaction {
    pub fn new(inner: web3::types::Transaction, r#type: TransactionType) -> Self {
        Self { inner, r#type }
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
        self.inner.input.0.clone()
    }

    pub fn transaction_type(&self) -> TransactionType {
        if self.input().starts_with(TRANSFER_SIGNATURE) && self.input().len() == 68 {
            TransactionType::ERC20
        } else {
            TransactionType::Other
        }
    }
}
