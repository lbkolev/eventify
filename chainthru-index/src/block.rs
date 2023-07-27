use ethereum_types::{H256, H64, U256, U64};
use web3::types::Transaction;

pub struct Block {
    inner: web3::types::Block<Transaction>,
}

impl Block {
    pub fn new(inner: web3::types::Block<Transaction>) -> Self {
        Self { inner }
    }

    pub fn hash(&self) -> H256 {
        self.inner.hash.unwrap_or(H256::zero())
    }

    

    pub async fn insert(&self, db_conn: &sqlx::PgPool) -> Result<(), sqlx::Error> {
        sqlx::query(
        "INSERT INTO block
        (hash, parent_hash, uncles_hash, author, state_root, transactions_root, receipts_root, number, gas_used, gas_limit, base_fee_per_gas, timestamp, difficulty, total_difficulty, transactions, size, nonce)
        VALUES
        ($1, $2, $3, $4, $5, $6, $7, $8, $9::numeric, $10::numeric, $11::numeric, $12::bigint, $13::numeric, $14::numeric, $15, $16, $17)",
    )
        .bind(self.inner.hash.unwrap_or(H256::zero()).as_bytes())
        .bind(self.inner.parent_hash.as_bytes())
        .bind(self.inner.uncles_hash.as_bytes())
        .bind(self.inner.author.as_bytes())
        .bind(self.inner.state_root.as_bytes())
        .bind(self.inner.transactions_root.as_bytes())
        .bind(self.inner.receipts_root.as_bytes())
        .bind(self.inner.number.unwrap_or(U64::zero()).as_u64() as i64)
        .bind(self.inner.gas_used.to_string())
        .bind(self.inner.gas_limit.to_string())
        .bind(self.inner.base_fee_per_gas.unwrap_or(U256::zero()).to_string())
        .bind(self.inner.timestamp.to_string())
        .bind(self.inner.difficulty.to_string())
        .bind(self.inner.total_difficulty.unwrap_or(U256::zero()).to_string())
        .bind(self.inner.transactions.len() as i64)
        .bind(self.inner.size.unwrap_or(U256::zero()).as_u64() as i64)
        .bind(self.inner.nonce.unwrap_or(H64::zero()).as_bytes())
        .execute(db_conn).await?;

        Ok(())
    }

    pub async fn insert_where(
        &self,
        db_conn: &sqlx::PgPool,
        where_clause: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            &format!("INSERT INTO block
            (hash, parent_hash, uncles_hash, author, state_root, transactions_root, receipts_root, number, gas_used, gas_limit, base_fee_per_gas, timestamp, difficulty, total_difficulty, transactions, size, nonce)
            VALUES
            ($1, $2, $3, $4, $5, $6, $7, $8, $9::numeric, $10::numeric, $11::numeric, $12::bigint, $13::numeric, $14::numeric, $15, $16, $17) WHERE {}", where_clause),
        )
            .bind(self.inner.hash.unwrap_or(H256::zero()).as_bytes())
            .bind(self.inner.parent_hash.as_bytes())
            .bind(self.inner.uncles_hash.as_bytes())
            .bind(self.inner.author.as_bytes())
            .bind(self.inner.state_root.as_bytes())
            .bind(self.inner.transactions_root.as_bytes())
            .bind(self.inner.receipts_root.as_bytes())
            .bind(self.inner.number.unwrap_or(U64::zero()).as_u64() as i64)
            .bind(self.inner.gas_used.to_string())
            .bind(self.inner.gas_limit.to_string())
            .bind(self.inner.base_fee_per_gas.unwrap_or(U256::zero()).to_string())
            .bind(self.inner.timestamp.to_string())
            .bind(self.inner.difficulty.to_string())
            .bind(self.inner.total_difficulty.unwrap_or(U256::zero()).to_string())
            .bind(self.inner.transactions.len() as i64)
            .bind(self.inner.size.unwrap_or(U256::zero()).as_u64() as i64)
            .bind(self.inner.nonce.unwrap_or(H64::zero()).as_bytes())
            .execute(db_conn).await?;

        Ok(())
    }
}
