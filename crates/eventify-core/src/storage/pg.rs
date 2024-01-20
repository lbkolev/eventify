#![allow(clippy::option_map_unit_fn)]

use sqlx::pool::PoolOptions;
use tracing::debug;

use crate::{error::StorageClientError, storage::Postgres, Auth, Error, StorageClient};
use eventify_primitives::{Contract, EthBlock, EthLog, EthTransaction};

impl Auth for Postgres {
    async fn connect(url: &str) -> Self {
        Self {
            inner: PoolOptions::new()
                .acquire_timeout(std::time::Duration::from_secs(2))
                .connect_lazy(url)
                .map_err(Error::from)
                .expect("Failed to connect to Postgres"),
        }
    }
}

impl Postgres {
    pub async fn new(url: &str) -> Self {
        Self::connect(url).await
    }
}

impl StorageClient for Postgres {
    async fn store_block(&self, block: &EthBlock) -> Result<(), Error> {
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
            .bind(block.nonce.map(|v| v.as_slice().to_vec()))
            .bind(block.base_fee.map(|v| v.to::<i64>()))
            .bind(block.parent_beacon_root.map(|v| v.as_slice().to_vec()))
            .bind(block.blob_gas_used.map(|v| v.to::<i64>()))
            .bind(block.excess_blob_gas.map(|v| v.to::<i64>()))
            .bind(block.withdrawals_hash.map(|v| v.as_slice().to_vec()))
            .bind(block.hash.map(|h| h.as_slice().to_vec().to_vec()))
            .execute(&self.inner)
            .await?;

        debug!(target: "eventify::idx::block", hash=?block.hash, number=?block.number, "Insert");
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
            .bind(tx.block_hash.map(|h| h.as_slice().to_vec().to_vec()))
            .bind(tx.block_number.map(|v| v.to::<i64>()))
            .bind(tx.from.as_slice())
            .bind(tx.gas.as_le_slice())
            .bind(tx.gas_price.as_le_slice())
            .bind(tx.hash.as_slice())
            .bind(tx.input.to_vec())
            .bind(tx.nonce.as_le_slice())
            .bind(tx.to.map(|v| v.as_slice().to_vec()))
            .bind(tx.transaction_index.map(|v| v.to::<i64>()))
            .bind(tx.value.as_le_slice())
            .bind(tx.v.as_le_slice())
            .bind(tx.r.as_le_slice())
            .bind(tx.s.as_le_slice())
            .execute(&self.inner)
            .await?;

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
            .map_err(|_| StorageClientError::StoreContract(tx.transaction_hash))?;

        debug!(target: "eventify::idx::contract", tx_hash=?tx.transaction_hash, tx_from=?tx.from, "Insert");
        Ok(())
    }

    async fn store_log(&self, _log: &EthLog) -> Result<(), Error> {
        //let sql = r#"INSERT INTO eth.log (
        //    address,
        //    topic0,
        //    topic1,
        //    topic2,
        //    topic3,
        //    data,
        //    block_hash,
        //    block_number,
        //    tx_hash,
        //    tx_index,
        //    tx_log_index,
        //    log_index,
        //    log_type,
        //    removed
        //    ) VALUES (
        //        $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14
        //    ) ON CONFLICT DO NOTHING"#;

        //let mut transaction_log_index_slice = [0u8; 32];
        //log.transaction_log_index
        //    .map(|v| v.to_big_endian(&mut transaction_log_index_slice));

        //let mut log_index_slice = [0u8; 32];
        //log.log_index.map(|v| v.to_big_endian(&mut log_index_slice));

        //sqlx::query(sql)
        //    .bind(log.address.as_ref())
        //    .bind(log.topics.first().map(|h| h.as_bytes()))
        //    .bind(log.topics.get(1).map(|h| h.as_bytes()))
        //    .bind(log.topics.get(2).map(|h| h.as_bytes()))
        //    .bind(log.topics.get(3).map(|h| h.as_bytes()))
        //    .bind(log.data.0.as_ref())
        //    .bind(log.block_hash.as_ref().map(|h| h.as_bytes()))
        //    .bind(log.block_number.map(|v| v.as_u64() as i32))
        //    .bind(log.transaction_hash.as_ref().map(|h| h.as_bytes()))
        //    .bind(log.transaction_index.map(|v| v.as_u64() as i32))
        //    .bind(transaction_log_index_slice)
        //    .bind(log_index_slice)
        //    .bind(log.log_type.as_ref())
        //    .bind(log.removed)
        //    .execute(&self.inner)
        //    .await
        //    .map_err(|_| StorageClientError::StoreLog(log.transaction_hash.unwrap()))?;

        //debug!(target: "eventify::idx::log", address=?log.address, block=?log.block_number, event=?log.topics.first().map(|h| format!("{:x}", h)) ,"Insert");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::clients::StorageClient;

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
        let db = super::Postgres { inner: pool };

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

        let block = serde_json::from_value::<EthBlock>(json).unwrap();
        println!("{:?}", block);
        db.store_block(&block).await?;

        teardown_test_db(db.inner, &db_name).await.unwrap();
    }

    #[tokio::test]
    async fn test_store_transaction() {
        let (pool, db_name) = setup_test_db().await.unwrap();
        let db = super::Postgres { inner: pool };

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
        let db = super::Postgres { inner: pool };

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
        let db = super::Postgres { inner: pool };

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
