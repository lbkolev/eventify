use alloy_primitives::U64;
use eyre::Result;
use redis::AsyncCommands;
use sqlx::{prelude::FromRow, Error as SqlError};
use utoipa::ToSchema;

use crate::{
    networks::{core::CoreLog, LogKind, NetworkKind, ResourceKind},
    traits::{Emit, Insert, Log},
    EmitError,
};

#[derive(
    Clone,
    Debug,
    Default,
    serde::Deserialize,
    serde::Serialize,
    PartialEq,
    Eq,
    Hash,
    FromRow,
    ToSchema,
)]
pub struct ZksyncLog {
    #[serde(flatten)]
    core: CoreLog,

    #[serde(rename = "l1BatchNumber")]
    pub l1_batch_number: Option<U64>,
    #[serde(rename = "transactionLogIndex")]
    pub tx_log_index: Option<U64>,
    #[serde(rename = "logType")]
    pub log_type: Option<String>,
}

impl Insert for ZksyncLog {
    async fn insert(
        &self,
        pool: &sqlx::PgPool,
        _: &Option<alloy_primitives::B256>,
    ) -> Result<(), SqlError> {
        let (
            address,
            block_hash,
            block_number,
            data,
            log_index,
            removed,
            topic0,
            topic1,
            topic2,
            topic3,
            tx_index,
            tx_hash,
        ) = self.core().db_repr();

        let l1_batch_number = self.l1_batch_number.map(|v| v.to::<i64>());
        let tx_log_index = self.tx_log_index.map(|v| v.to::<i64>());

        let query = r#"
            INSERT INTO log (
                network,
                address,
                block_hash,
                block_number,
                data,
                log_index,
                removed,
                topic0,
                topic1,
                topic2,
                topic3,
                tx_index,
                tx_hash,

                l1_batch_number,
                tx_log_index,
                log_type
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)
            ON CONFLICT DO NOTHING
            "#;

        sqlx::query(query)
            .bind(NetworkKind::Zksync)
            .bind(address)
            .bind(block_hash)
            .bind(block_number)
            .bind(data)
            .bind(log_index)
            .bind(removed)
            .bind(topic0)
            .bind(topic1)
            .bind(topic2)
            .bind(topic3)
            .bind(tx_index)
            .bind(tx_hash)
            .bind(l1_batch_number)
            .bind(tx_log_index)
            .bind(&self.log_type)
            .execute(pool)
            .await?;

        Ok(())
    }
}

impl Emit for ZksyncLog {
    async fn emit(
        &self,
        queue: &redis::Client,
        network: &crate::networks::NetworkKind,
    ) -> eyre::Result<(), EmitError> {
        let mut con = queue.get_async_connection().await?;

        let channel = format!("{}:{}", network, ResourceKind::Log(LogKind::Raw));
        con.lpush(channel, serde_json::to_string(self)?).await?;

        Ok(())
    }
}

impl Log for ZksyncLog {
    fn core(&self) -> &CoreLog {
        &self.core
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_zksync_log() {
        let json = serde_json::json!({
          "address": "0x000000000000000000000000000000000000800a",
          "topics": [
            "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef",
            "0x0000000000000000000000000000000000000000000000000000000000008001",
            "0x00000000000000000000000056ddd604011c5f8629bd7c2472e3504bd32c269b"
          ],
          "data": "0x00000000000000000000000000000000000000000000000000014c39ba59ba00",
          "blockHash": "0x13f57e3e974d3e5bfe80e4588e6865e008d9c7a72ba55104924f1c9ee6999185",
          "blockNumber": "0x11451a",
          "l1BatchNumber": null,
          "transactionHash": "0xea856ad27c508a5824dc0399fd0a412e88213c496ac4e50549bbdd62619a952e",
          "transactionIndex": "0x0",
          "logIndex": "0x5",
          "transactionLogIndex": "0x5",
          "logType": null,
          "removed": false
        });

        assert!(serde_json::from_value::<ZksyncLog>(json).is_ok());
    }

    #[test]
    fn test_deserialize_empty_zksync_log() {
        let json = serde_json::json!({});

        assert!(serde_json::from_value::<ZksyncLog>(json).is_err());
    }
}
