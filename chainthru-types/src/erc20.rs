use derive_builder::Builder;
use ethereum_types::{H160, H256, U256};
use web3::types::Transaction;

use crate::contract_func;

contract_func!(Transfer(
    contract: H160,
    hash: H256,
    from: H160,
    to: H160,
    value: U256
));

impl From<Transaction> for Transfer {
    fn from(transaction: Transaction) -> Self {
        Self {
            contract: transaction.to.unwrap_or(H160::default()),
            hash: transaction.hash,
            from: transaction.from.unwrap_or(H160::default()),
            to: H160::from_slice(&transaction.input.0[16..36]),
            value: U256::from(&transaction.input.0[36..68]),
        }
    }
}

impl Transfer {
    async fn insert(&self, db_conn: &sqlx::PgPool) -> Result<()> {
        let sql = format!(
            "
            INSERT INTO erc20.transfer (contract, transaction_hash, send_from, send_to, value)  
            VALUES ($1, $2, $3, $4, $5::numeric)"
        );

        sqlx::query(sql.as_str())
            .bind(self.contract.as_bytes())
            .bind(self.hash.as_bytes())
            .bind(self.from.as_bytes())
            .bind(self.to.as_bytes())
            .bind(self.value.to_string())
            .execute(db_conn)
            .await?;

        Ok(())
    }

    async fn insert_where(&self, db_conn: &sqlx::PgPool, where_clause: &str) -> Result<()> {
        let sql = format!(
            "
            INSERT INTO erc20.transfer (contract, transaction_hash, send_from, send_to, value) 
            VALUES ($1, $2, $3, $4, $5::numeric)
            WHERE {}",
            where_clause
        );

        sqlx::query(sql.as_str())
            .bind(self.contract.as_bytes())
            .bind(self.hash.as_bytes())
            .bind(self.from.as_bytes())
            .bind(self.to.as_bytes())
            .bind(self.value.to_string())
            .execute(db_conn)
            .await?;

        Ok(())
    }
}
