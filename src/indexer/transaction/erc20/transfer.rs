use ethereum_types::{H160, H256, U256};

#[derive(Debug, Default)]
pub struct Transfer {
    pub hash: H256,
    pub from: H160,
    pub to: H160,
    pub value: U256,
}

impl Transfer {
    pub fn new(hash: H256, from: H160, to: H160, value: U256) -> Self {
        Self {
            hash,
            from,
            to,
            value,
        }
    }

    pub async fn insert(&self, contract: H160, db_conn: &sqlx::PgPool) -> Result<(), sqlx::Error> {
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
}
