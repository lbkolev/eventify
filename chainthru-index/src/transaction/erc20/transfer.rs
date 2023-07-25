use async_trait::async_trait;
use derive_builder::Builder;
use ethereum_types::{H160, H256, U256};

use crate::transaction::DBInsert;

#[derive(Builder, Clone, Debug, Default)]
pub struct Transfer {
    /// The transaction hash
    pub hash: H256,

    /// The sender of the transaction
    pub from: H160,

    /// The receiver of the transfer
    pub to: H160,

    /// The amount of tokens transferred from the sender to the receiver
    pub value: U256,
}

impl Transfer {
    /// Create a new instance of the transfer
    pub fn new(hash: H256, from: H160, to: H160, value: U256) -> Self {
        Self {
            hash,
            from,
            to,
            value,
        }
    }

    /// Create a new instance of the transfer from a transaction
    pub fn from_transaction(transaction: &web3::types::Transaction) -> Self {
        Self {
            hash: transaction.hash,
            from: transaction.from.unwrap_or(H160::zero()),
            to: H160::from_slice(&transaction.input.0[16..36]),
            value: U256::from(&transaction.input.0[36..68]),
        }
    }
}

#[async_trait]
impl DBInsert for Transfer {
    async fn insert(&self, contract: H160, db_conn: &sqlx::PgPool) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO erc20.transfer (contract, transaction_hash, send_from, send_to, value) VALUES ($1, $2, $3, $4, $5::numeric)",
        )
            .bind(contract.as_bytes())
            .bind(self.hash.as_bytes())
            .bind(self.from.as_bytes())
            .bind(self.to.as_bytes())
            .bind(self.value.to_string())
            .execute(db_conn).await?;

        Ok(())
    }

    async fn insert_where(
        &self,
        contract: H160,
        db_conn: &sqlx::PgPool,
        where_clause: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            &format!("INSERT INTO erc20.transfer (contract, transaction_hash, send_from, send_to, value) VALUES ($1, $2, $3, $4, $5::numeric) WHERE {}", where_clause),
        )
            .bind(contract.as_bytes())
            .bind(self.hash.as_bytes())
            .bind(self.from.as_bytes())
            .bind(self.to.as_bytes())
            .bind(self.value.to_string())
            .execute(db_conn).await?;

        Ok(())
    }
}
