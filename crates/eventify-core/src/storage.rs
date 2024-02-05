#![allow(clippy::option_map_unit_fn)]

use alloy_primitives::{Address, Bytes, FixedBytes, B256, U64};
use eventify_configs::database::DatabaseConfig;
use sqlx::{pool::PoolOptions, postgres::Postgres, Pool};
use tracing::debug;

use crate::{Error, Store};
use eventify_primitives::network::{Contract, EthBlock, EthLog, EthTransaction};

#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum StoreError {
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
    inner: Pool<Postgres>,
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
        write!(
            f,
            "{}: [{}]",
            stringify!(Storage),
            stringify!(Pool<Postgres>)
        )
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

impl Store for Storage {
    async fn store_block(&self, block: &EthBlock<B256>) -> Result<(), Error> {
        let sql = r#"INSERT INTO eth.block (
                parent_hash,
                uncles_hash,
                coinbase,
                root,
                tx_hash,
                receipt_hash,
                difficulty,
                number,
                gas_limit,
                gas_used,
                time,
                extra,
                mix_digest,
                nonce,
                base_fee,
                parent_beacon_root,
                blob_gas_used,
                excess_blob_gas,
                withdraws_hash,
                hash
                ) VALUES (
                    $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20
                ) ON CONFLICT DO NOTHING"#;

        sqlx::query(sql)
            .bind(block.parent_hash.as_slice())
            .bind(block.uncle_hash.as_slice())
            .bind(block.coinbase.as_slice())
            .bind(block.root.as_slice())
            .bind(block.tx_hash.as_slice())
            .bind(block.receipt_hash.as_slice())
            .bind(block.difficulty.as_le_slice())
            .bind(block.number.map(|v| v.to::<i64>()))
            .bind(block.gas_limit.as_le_slice())
            .bind(block.gas_used.as_le_slice())
            .bind(block.time.to::<i64>())
            .bind(block.extra.to_vec())
            .bind(block.mix_digest.as_slice())
            .bind(block.nonce.as_ref().map(|v| v.as_slice()))
            .bind(block.base_fee.map(|v| v.to::<i64>()))
            .bind(block.parent_beacon_root.as_ref().map(|v| v.as_slice()))
            .bind(block.blob_gas_used.map(|v| v.to::<i64>()))
            .bind(block.excess_blob_gas.map(|v| v.to::<i64>()))
            .bind(block.withdrawals_hash.as_ref().map(|v| v.as_slice()))
            .bind(block.hash.as_ref().map(|v| v.as_slice()))
            .execute(&self.inner)
            .await
            .map_err(|e| StoreError::StoreBlockFailed {
                hash: block.hash.expect("unable to get block hash"),
                err: e.to_string(),
            })?;

        debug!(target: "eventify::core::store::block", hash=?block.hash, number=?block.number);
        Ok(())
    }

    async fn store_transaction(&self, tx: &EthTransaction) -> Result<(), crate::Error> {
        let sql = r#"INSERT INTO eth.transaction (
            block_hash,
            block_number,
            "from",
            gas,
            gas_price,
            hash,
            input,
            nonce,
            "to",
            transaction_index,
            value,
            v, r, s) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14
            ) ON CONFLICT DO NOTHING"#;

        sqlx::query(sql)
            .bind(tx.block_hash.as_ref().map(|v| v.as_slice()))
            .bind(tx.block_number.map(|v| v.to::<i64>()))
            .bind(tx.from.as_slice())
            .bind(tx.gas.as_le_slice())
            .bind(tx.gas_price.as_le_slice())
            .bind(tx.hash.as_slice())
            .bind(tx.input.to_vec())
            .bind(tx.nonce.as_le_slice())
            .bind(tx.to.as_ref().map(|v| v.as_slice()))
            .bind(tx.transaction_index.map(|v| v.to::<i64>()))
            .bind(tx.value.as_le_slice())
            .bind(tx.v.as_le_slice())
            .bind(tx.r.as_le_slice())
            .bind(tx.s.as_le_slice())
            .execute(&self.inner)
            .await
            .map_err(|e| StoreError::StoreTransactionFailed {
                hash: tx.hash,
                err: e.to_string(),
            })?;

        debug!(target: "eventify::core::store::transaction", hash=?tx.hash);
        Ok(())
    }

    async fn store_log(&self, log: &EthLog) -> Result<(), Error> {
        let sql = r#"INSERT INTO eth.log (
            address,
            topic0,
            topic1,
            topic2,
            topic3,
            data,
            block_hash,
            block_number,
            tx_hash,
            tx_index,
            log_index,
            removed
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12
            ) ON CONFLICT (address, block_hash, tx_hash) DO NOTHING"#;

        sqlx::query(sql)
            .bind(log.address.as_slice())
            .bind(log.topics.first().map(|v| v.as_slice()))
            .bind(log.topics.get(1).map(|v| v.as_slice()))
            .bind(log.topics.get(2).map(|v| v.as_slice()))
            .bind(log.topics.get(3).map(|v| v.as_slice()))
            .bind(log.data.0.as_ref())
            .bind(log.block_hash.as_ref().map(|v| v.as_slice()))
            .bind(log.block_number.map(|v| v.to::<i64>()))
            .bind(log.transaction_hash.as_ref().map(|v| v.as_slice()))
            .bind(log.transaction_index.map(|v| v.to::<i64>()))
            .bind(log.log_index.map(|v| v.to::<i64>()))
            .bind(log.removed)
            .execute(&self.inner)
            .await
            .map_err(|e| StoreError::StoreLogFailed {
                addr: log.address,
                err: e.to_string(),
            })?;

        debug!(target: "eventify::core::store::log", address=?log.address, block=?log.block_number, event=?log.topics.first());
        Ok(())
    }

    async fn store_contract(&self, tx: &Contract) -> Result<(), Error> {
        let sql = r#"INSERT INTO eth.contract (
            tx_hash,
            "from",
            input
            ) VALUES (
                $1, $2, $3
            ) ON CONFLICT DO NOTHING"#;

        sqlx::query(sql)
            .bind(tx.transaction_hash.as_slice())
            .bind(tx.from.as_slice())
            .bind(tx.input.0.to_vec())
            .execute(&self.inner)
            .await
            .map_err(|e| StoreError::StoreContractFailed {
                hash: tx.transaction_hash,
                err: e.to_string(),
            })?;

        debug!(target: "eventify::core::store::contract", tx_hash=?tx.transaction_hash, tx_from=?tx.from);
        Ok(())
    }

    async fn store_log_transfer(
        &self,
        tx_hash: &FixedBytes<32>,
        from: &FixedBytes<32>,
        to: &FixedBytes<32>,
        value: Bytes,
    ) -> Result<(), Error> {
        let sql = r#"INSERT INTO eth.log_transfer (
            tx_hash,
            "from",
            "to",
            value )
            VALUES (
                $1, $2, $3, $4
            ) ON CONFLICT DO NOTHING"#;

        sqlx::query(sql)
            .bind(tx_hash.as_slice())
            .bind(from.as_slice())
            .bind(to.as_slice())
            .bind(value.to_vec())
            .execute(&self.inner)
            .await?;

        debug!(target: "eventify::core::store::eth_transfer", from=?from, to=?to, value=?value);
        Ok(())
    }

    async fn store_log_approval(
        &self,
        tx_hash: &FixedBytes<32>,
        owner: &FixedBytes<32>,
        spender: &FixedBytes<32>,
        value: Bytes,
    ) -> Result<(), Error> {
        let sql = r#"INSERT INTO eth.log_approval (
            tx_hash,
            owner,
            spender,
            value )
            VALUES (
                $1, $2, $3, $4
            ) ON CONFLICT DO NOTHING"#;

        sqlx::query(sql)
            .bind(tx_hash.as_slice())
            .bind(owner.as_slice())
            .bind(spender.as_slice())
            .bind(value.to_vec())
            .execute(&self.inner)
            .await?;

        debug!(target: "eventify::core::store::eth_approval", owner=?owner, spender=?spender, value=?value);
        Ok(())
    }

    async fn store_log_approval_for_all(
        &self,
        tx_hash: &FixedBytes<32>,
        owner: &FixedBytes<32>,
        operator: &FixedBytes<32>,
        approved: bool,
    ) -> Result<(), Error> {
        let sql = r#"INSERT INTO eth.log_approval_for_all (
            tx_hash,
            owner,
            operator,
            approved )
            VALUES (
                $1, $2, $3, $4
            ) ON CONFLICT DO NOTHING"#;

        sqlx::query(sql)
            .bind(tx_hash.as_slice())
            .bind(owner.as_slice())
            .bind(operator.as_slice())
            .bind(approved)
            .execute(&self.inner)
            .await?;

        debug!(target: "eventify::core::store::eth_approval_for_all", owner=?owner, operator=?operator, approved=?approved);
        Ok(())
    }

    async fn store_log_sent(
        &self,
        tx_hash: &FixedBytes<32>,
        operator: &FixedBytes<32>,
        from: &FixedBytes<32>,
        to: &FixedBytes<32>,
        amount: Bytes,
        data: Bytes,
        operator_data: Bytes,
    ) -> Result<(), Error> {
        let sql = r#"INSERT INTO eth.log_sent (
            tx_hash,
            operator,
            from,
            to,
            amount,
            data,
            operator_data )
            VALUES (
                $1, $2, $3, $4, $5, $6, $7
            ) ON CONFLICT DO NOTHING"#;

        sqlx::query(sql)
            .bind(tx_hash.as_slice())
            .bind(operator.as_slice())
            .bind(from.as_slice())
            .bind(to.as_slice())
            .bind(amount.to_vec())
            .bind(data.to_vec())
            .bind(operator_data.to_vec())
            .execute(&self.inner)
            .await?;

        debug!(target: "eventify::core::store::eth_sent", operator=?operator, from=?from, to=?to, amount=?amount);
        Ok(())
    }

    async fn store_log_minted(
        &self,
        tx_hash: &FixedBytes<32>,
        operator: &FixedBytes<32>,
        to: &FixedBytes<32>,
        amount: Bytes,
        data: Bytes,
        operator_data: Bytes,
    ) -> Result<(), Error> {
        let sql = r#"INSERT INTO eth.log_minted (
            tx_hash,
            operator,
            to,
            amount,
            data,
            operator_data )
            VALUES (
                $1, $2, $3, $4, $5, $6
            ) ON CONFLICT DO NOTHING"#;

        sqlx::query(sql)
            .bind(tx_hash.as_slice())
            .bind(operator.as_slice())
            .bind(to.as_slice())
            .bind(amount.to_vec())
            .bind(data.to_vec())
            .bind(operator_data.to_vec())
            .execute(&self.inner)
            .await?;

        debug!(target: "eventify::core::store::eth_minted", operator=?operator, to=?to, amount=?amount);
        Ok(())
    }

    async fn store_log_burned(
        &self,
        tx_hash: &FixedBytes<32>,
        operator: &FixedBytes<32>,
        from: &FixedBytes<32>,
        amount: Bytes,
        data: Bytes,
        operator_data: Bytes,
    ) -> Result<(), Error> {
        let sql = r#"INSERT INTO eth.log_burned (
            tx_hash,
            operator,
            from,
            amount,
            data,
            operator_data )
            VALUES (
                $1, $2, $3, $4, $5, $6
            ) ON CONFLICT DO NOTHING"#;

        sqlx::query(sql)
            .bind(tx_hash.as_slice())
            .bind(operator.as_slice())
            .bind(from.as_slice())
            .bind(amount.to_vec())
            .bind(data.to_vec())
            .bind(operator_data.to_vec())
            .execute(&self.inner)
            .await?;

        debug!(target: "eventify::core::store::eth_burned", operator=?operator, from=?from, amount=?amount);
        Ok(())
    }

    async fn store_log_authorized_operator(
        &self,
        tx_hash: &FixedBytes<32>,
        operator: &FixedBytes<32>,
        holder: &FixedBytes<32>,
    ) -> Result<(), Error> {
        let sql = r#"INSERT INTO eth.log_authorized_operator (
            tx_hash,
            operator,
            holder )
            VALUES (
                $1, $2, $3
            ) ON CONFLICT DO NOTHING"#;

        sqlx::query(sql)
            .bind(tx_hash.as_slice())
            .bind(operator.as_slice())
            .bind(holder.as_slice())
            .execute(&self.inner)
            .await?;

        debug!(target: "eventify::core::store::eth_authorized_operator", operator=?operator, holder=?holder);
        Ok(())
    }

    async fn store_log_revoked_operator(
        &self,
        tx_hash: &FixedBytes<32>,
        operator: &FixedBytes<32>,
        holder: &FixedBytes<32>,
    ) -> Result<(), Error> {
        let sql = r#"INSERT INTO eth.log_revoked_operator (
            tx_hash,
            operator,
            holder )
            VALUES (
                $1, $2, $3
            ) ON CONFLICT DO NOTHING"#;

        sqlx::query(sql)
            .bind(tx_hash.as_slice())
            .bind(operator.as_slice())
            .bind(holder.as_slice())
            .execute(&self.inner)
            .await?;

        debug!(target: "eventify::core::store::eth_revoked_operator", operator=?operator, holder=?holder);
        Ok(())
    }

    async fn store_log_transfer_single(
        &self,
        tx_hash: &FixedBytes<32>,
        operator: &FixedBytes<32>,
        from: &FixedBytes<32>,
        to: &FixedBytes<32>,
        id: U64,
        value: Bytes,
    ) -> Result<(), Error> {
        let sql = r#"INSERT INTO eth.log_transfer_single (
            tx_hash,
            operator,
            from,
            to,
            id,
            value )
            VALUES (
                $1, $2, $3, $4, $5, $6
            ) ON CONFLICT DO NOTHING"#;

        sqlx::query(sql)
            .bind(tx_hash.as_slice())
            .bind(operator.as_slice())
            .bind(from.as_slice())
            .bind(to.as_slice())
            .bind(id.to::<i64>())
            .bind(value.to_vec())
            .execute(&self.inner)
            .await?;

        debug!(target: "eventify::core::store::eth_transfer_single", operator=?operator, from=?from, to=?to, id=?id, value=?value);
        Ok(())
    }

    async fn store_log_transfer_batch(
        &self,
        tx_hash: &FixedBytes<32>,
        operator: &FixedBytes<32>,
        from: &FixedBytes<32>,
        to: &FixedBytes<32>,
        ids: Vec<U64>,
        values: Vec<Bytes>,
    ) -> Result<(), Error> {
        let sql = r#"INSERT INTO eth.log_transfer_batch (
            tx_hash,
            operator,
            from,
            to,
            ids,
            values )
            VALUES (
                $1, $2, $3, $4, $5, $6
            ) ON CONFLICT DO NOTHING"#;

        sqlx::query(sql)
            .bind(tx_hash.as_slice())
            .bind(operator.as_slice())
            .bind(from.as_slice())
            .bind(to.as_slice())
            .bind(ids.iter().map(|v| v.to::<i64>()).collect::<Vec<_>>())
            .bind(values.iter().map(|v| v.to_vec()).collect::<Vec<_>>())
            .execute(&self.inner)
            .await?;

        debug!(target: "eventify::core::store::eth_transfer_batch", operator=?operator, from=?from, to=?to, ids=?ids, values=?values);
        Ok(())
    }

    async fn store_log_uri(
        &self,
        tx_hash: &FixedBytes<32>,
        uri: String,
        id: U64,
    ) -> Result<(), Error> {
        let sql = r#"INSERT INTO eth.log_uri (
            tx_hash,
            uri,
            id )
            VALUES (
                $1, $2, $3
            ) ON CONFLICT DO NOTHING"#;

        sqlx::query(sql)
            .bind(tx_hash.as_slice())
            .bind(uri)
            .bind(id.to::<i64>())
            .execute(&self.inner)
            .await?;

        //debug!(target: "eventify::core::store::eth_uri", tx_hash=?tx_hash, uri=?uri, id=?id);
        Ok(())
    }

    async fn store_log_deposit(
        &self,
        tx_hash: &FixedBytes<32>,
        sender: &FixedBytes<32>,
        owner: &FixedBytes<32>,
        assets: U64,
        shares: U64,
    ) -> Result<(), Error> {
        let sql = r#"INSERT INTO eth.log_deposit (
            tx_hash,
            sender,
            owner,
            assets,
            shares )
            VALUES (
                $1, $2, $3, $4, $5
            ) ON CONFLICT DO NOTHING"#;

        sqlx::query(sql)
            .bind(tx_hash.as_slice())
            .bind(sender.as_slice())
            .bind(owner.as_slice())
            .bind(assets.to::<i64>())
            .bind(shares.to::<i64>())
            .execute(&self.inner)
            .await?;

        debug!(target: "eventify::core::store::eth_deposit", tx_hash=?tx_hash, sender=?sender, owner=?owner, assets=?assets, shares=?shares);
        Ok(())
    }

    async fn store_log_withdraw(
        &self,
        tx_hash: &FixedBytes<32>,
        sender: &FixedBytes<32>,
        receiver: &FixedBytes<32>,
        owner: &FixedBytes<32>,
        assets: U64,
        shares: U64,
    ) -> Result<(), Error> {
        let sql = r#"INSERT INTO eth.log_withdraw (
            tx_hash,
            sender,
            receiver,
            owner,
            assets,
            shares )
            VALUES (
                $1, $2, $3, $4, $5, $6
            ) ON CONFLICT DO NOTHING"#;

        sqlx::query(sql)
            .bind(tx_hash.as_slice())
            .bind(sender.as_slice())
            .bind(receiver.as_slice())
            .bind(owner.as_slice())
            .bind(assets.to::<i64>())
            .bind(shares.to::<i64>())
            .execute(&self.inner)
            .await?;

        debug!(target: "eventify::core::store::eth_withdraw", tx_hash=?tx_hash, owner=?owner, assets=?assets, shares=?shares);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
    use uuid::Uuid;

    async fn setup_test_db() -> std::result::Result<(Pool<Postgres>, String), sqlx::Error> {
        dotenv::dotenv().ok();
        let db_url = "postgres://postgres:password@localhost:5432/";
        let master_pool = PgPoolOptions::new()
            .connect(&format!("{}postgres", db_url))
            .await?;

        let db_name = format!("test_{}", Uuid::new_v4().simple());
        let db_url = format!("{}{}", db_url, db_name);

        sqlx::query(&format!("CREATE DATABASE {}", db_name))
            .execute(&master_pool)
            .await?;

        let pool = PgPoolOptions::new().connect(&db_url).await?;

        sqlx::migrate!("../../migrations").run(&pool).await?;

        Ok((pool, db_name))
    }

    async fn teardown_test_db(
        pool: Pool<Postgres>,
        db_name: &str,
    ) -> std::result::Result<(), sqlx::Error> {
        // Disconnect all connections from the pool
        drop(pool);

        let database_url = "postgres://postgres:password@localhost:5432/postgres";
        let master_pool = PgPoolOptions::new().connect(database_url).await?;

        sqlx::query(&format!("DROP DATABASE IF EXISTS {}", db_name))
            .execute(&master_pool)
            .await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_store_block() {
        let (pool, db_name) = setup_test_db().await.unwrap();
        let db = Storage {
            inner: pool,
            config: DatabaseConfig::default(),
        };

        let json = serde_json::json!(
        {
            "baseFeePerGas": "0x7",
            "miner": "0x0000000000000000000000000000000000000001",
            "number": "0x1b4",
            "hash": "0x0e670ec64341771606e55d6b4ca35a1a6b75ee3d5145a99d05921026d1527331",
            "parentHash": "0x9646252be9520f6e71339a8df9c55e4d7619deeb018d2a3f2d21fc165dde5eb5",
            "unclesHash": "0x9646252be9520f6e71339a8df9c55e4d7619deeb018d2a3f2d21fc165dde5eb5",
            "mixHash": "0x1010101010101010101010101010101010101010101010101010101010101010",
            "nonce": "0x0000000000000000",
            "sealFields": [
              "0xe04d296d2460cfb8472af2c5fd05b5a214109c25688d3704aed5484f9a7792f2",
              "0x0000000000000042"
            ],
            "sha3Uncles": "0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347",
            "logsBloom":  "0x0e670ec64341771606e55d6b4ca35a1a6b75ee3d5145a99d05921026d15273310e670ec64341771606e55d6b4ca35a1a6b75ee3d5145a99d05921026d15273310e670ec64341771606e55d6b4ca35a1a6b75ee3d5145a99d05921026d15273310e670ec64341771606e55d6b4ca35a1a6b75ee3d5145a99d05921026d15273310e670ec64341771606e55d6b4ca35a1a6b75ee3d5145a99d05921026d15273310e670ec64341771606e55d6b4ca35a1a6b75ee3d5145a99d05921026d15273310e670ec64341771606e55d6b4ca35a1a6b75ee3d5145a99d05921026d15273310e670ec64341771606e55d6b4ca35a1a6b75ee3d5145a99d05921026d1527331",
            "transactionsRoot": "0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421",
            "receiptsRoot": "0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421",
            "stateRoot": "0xd5855eb08b3387c0af375e9cdb6acfc05eb8f519e419b874b6ff2ffda7ed1dff",
            "difficulty": "0x27f07",
            "totalDifficulty": "0x27f07",
            "extraData": "0x0000000000000000000000000000000000000000000000000000000000000000",
            "size": "0x27f07",
            "gasLimit": "0x9f759",
            "minGasPrice": "0x9f759",
            "gasUsed": "0x9f759",
            "timestamp": "0x54e34e8e",
            "transactions": [],
            "uncles": []
          }
        );

        let block = serde_json::from_value::<EthBlock<B256>>(json).unwrap();
        println!("{:?}", block);
        db.store_block(&block).await.unwrap();

        teardown_test_db(db.inner, &db_name).await.unwrap();
    }

    #[tokio::test]
    async fn test_store_transaction() {
        let (pool, db_name) = setup_test_db().await.unwrap();
        let db = Storage {
            inner: pool,
            config: DatabaseConfig::default(),
        };

        let json = serde_json::json!({
            "blockHash":"0x1d59ff54b1eb26b013ce3cb5fc9dab3705b415a67127a003c3e61eb445bb8df2",
            "blockNumber":"0x5daf3b",
            "from":"0xa7d9ddbe1f17865597fbd27ec712455208b6b76d",
            "gas":"0xc350",
            "gasPrice":"0x4a817c800",
            "hash":"0x88df016429689c079f3b2f6ad39fa052532c56795b733da78a91ebe6a713944b",
            "input":"0x68656c6c6f21",
            "nonce":"0x15",
            "to":"0xf02c1c8e6114b1dbe8937a39260b5b0a374432bb",
            "transactionIndex":"0x41",
            "value":"0xf3dbb76162000",
            "v":"0x25",
            "r":"0x1b5e176d927f8e9ab405058b2d2457392da3e20f328b16ddabcebc33eaac5fea",
            "s":"0x4ba69724e8f69de52f0125ad8b3c5c2cef33019bac3249e2c0a2192766d1721c"
        });

        let tx = serde_json::from_value::<EthTransaction>(json).unwrap();
        println!("{:?}", tx);
        db.store_transaction(&tx).await.unwrap();

        teardown_test_db(db.inner, &db_name).await.unwrap();
    }

    #[tokio::test]
    async fn test_store_contract() {
        let (pool, db_name) = setup_test_db().await.unwrap();
        let db = Storage {
            inner: pool,
            config: DatabaseConfig::default(),
        };

        let json = serde_json::json!({
            "transactionHash":"0x1d59ff54b1eb26b013ce3cb5fc9dab3705b415a67127a003c3e61eb445bb8df2",
            "from":"0xa7d9ddbe1f17865597fbd27ec712455208b6b76d",
            "input":"0x68656c6c6f21"
        });

        let contract = serde_json::from_value::<Contract>(json).unwrap();
        println!("{:?}", contract);
        db.store_contract(&contract).await.unwrap();

        teardown_test_db(db.inner, &db_name).await.unwrap();
    }

    #[tokio::test]
    async fn test_store_log() {
        let (pool, db_name) = setup_test_db().await.unwrap();
        let db = Storage {
            inner: pool,
            config: DatabaseConfig::default(),
        };

        let json = serde_json::json!(
            {
            "address": "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2",
            "topics": [
                "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef",
                "0x000000000000000000000000a7ca2c8673bcfa5a26d8ceec2887f2cc2b0db22a",
                "0x00000000000000000000000006da0fd433c1a5d7a4faa01111c044910a184553"
            ],
            "data": "0x000000000000000000000000000000000000000000000000007c585087238000",
            "blockHash": "0x6624f87d3435cc938de6442db45e06f23582a7eeddb5ac15126d440db03e75f4",
            "blockNumber": "0x11d389d",
            "transactionHash": "0x933c80c2a18cbf64ec28662991186bd340519eb6974f3d301195b82064329fc8",
            "transactionIndex": "0xd5",
            "logIndex": "0x200",
            "transactionLogIndex": null,
            "logType": null,
            "removed": false
            }
        );

        let log = serde_json::from_value::<EthLog>(json).unwrap();
        println!("{:#?}", log);
        db.store_log(&log).await.unwrap();

        teardown_test_db(db.inner, &db_name).await.unwrap();
    }
}
