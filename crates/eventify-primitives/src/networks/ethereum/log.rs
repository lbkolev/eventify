use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

use alloy_primitives::{Address, BlockNumber, Bytes, B256, U64};
use eyre::Result;
use redis::Commands;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, Error as SqlError};
use utoipa::ToSchema;

use crate::{
    networks::{LogKind, ResourceKind},
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
pub struct EthLog {
    pub removed: bool,
    #[serde(rename = "logIndex")]
    pub log_index: Option<U64>,
    #[serde(rename = "transactionIndex")]
    pub transaction_index: Option<U64>,
    #[serde(rename = "transactionHash")]
    pub transaction_hash: Option<B256>,
    #[serde(rename = "blockHash")]
    pub block_hash: Option<B256>,
    #[serde(rename = "blockNumber")]
    pub block_number: Option<U64>,
    pub address: Address,
    pub data: Bytes,
    pub topics: Vec<B256>,
}

impl Insert for EthLog {
    async fn insert(
        &self,
        pool: &sqlx::PgPool,
        schema: &str,
        _: &Option<alloy_primitives::B256>,
    ) -> Result<(), SqlError> {
        let address = self.address.as_slice();
        let topic0 = self.topics.first().map(|v| v.as_slice());
        let topic1 = self.topics.get(1).map(|v| v.as_slice());
        let topic2 = self.topics.get(2).map(|v| v.as_slice());
        let topic3 = self.topics.get(3).map(|v| v.as_slice());
        let data = self.data.0.as_ref();
        let block_hash = self.block_hash.as_ref().map(|v| v.as_slice());
        let block_number = self.block_number.map(|v| v.to::<i64>());
        let tx_hash = self.transaction_hash.as_ref().map(|v| v.as_slice());
        let tx_index = self.transaction_index.map(|v| v.to::<i64>());
        let log_index = self.log_index.map(|v| v.to::<i64>());
        let removed = self.removed;

        let sql = format!(
            r#"INSERT INTO {schema}.log (
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
            ) ON CONFLICT (address, block_hash, tx_hash) DO NOTHING"#,
        );

        sqlx::query(&sql)
            .bind(address)
            .bind(topic0)
            .bind(topic1)
            .bind(topic2)
            .bind(topic3)
            .bind(data)
            .bind(block_hash)
            .bind(block_number)
            .bind(tx_hash)
            .bind(tx_index)
            .bind(log_index)
            .bind(removed)
            .execute(pool)
            .await?;

        Ok(())
    }
}

impl Emit for EthLog {
    async fn emit(
        &self,
        queue: &redis::Client,
        network: &crate::networks::NetworkKind,
    ) -> eyre::Result<(), EmitError> {
        let mut con = queue.get_connection()?;

        let channel = format!("{}:{}", network, ResourceKind::Log(LogKind::Raw));
        con.lpush(channel, serde_json::to_string(self)?)?;

        Ok(())
    }
}

impl Log for EthLog {
    fn block_hash(&self) -> Option<B256> {
        self.block_hash
    }
    fn block_number(&self) -> Option<U64> {
        self.block_number
    }

    fn tx_hash(&self) -> Option<B256> {
        self.transaction_hash
    }

    fn tx_index(&self) -> Option<U64> {
        self.transaction_index
    }

    fn topics(&self) -> &Vec<B256> {
        &self.topics
    }

    fn data(&self) -> &alloy_primitives::Bytes {
        &self.data
    }

    fn address(&self) -> &alloy_primitives::Address {
        &self.address
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Criteria {
    #[serde(rename = "fromBlock", serialize_with = "serialize_block_number")]
    pub from: BlockNumber,
    #[serde(rename = "toBlock", serialize_with = "serialize_block_number")]
    pub to: BlockNumber,
    pub address: Option<Vec<Address>>,
    pub topics: Option<Vec<B256>>,
}

impl Criteria {
    pub fn new(
        from: BlockNumber,
        to: BlockNumber,
        address: Option<Vec<Address>>,
        topics: Option<Vec<B256>>,
    ) -> Criteria {
        Criteria {
            from,
            to,
            address,
            topics,
        }
    }
}

fn serialize_block_number<S>(x: &BlockNumber, s: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let hex = format!("0x{:x}", x);
    s.serialize_str(&hex)
}

impl Display for Criteria {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

impl FromStr for Criteria {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

impl From<&str> for Criteria {
    fn from(s: &str) -> Self {
        serde_json::from_str(s).expect("failed to parse criteria")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_log() {
        let json = serde_json::json!(
            {
            "address": "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2",
            "topics": [
                "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef",
                "0x000000000000000000000000a7ca2c8673bcfa5a26d8ceec2887f2cc2b0db22a",
                "0x00000000000000000000000006da0fd433c1a5d7a4faa01111c044910a184553"
            ],
            "data": "0x000000000000000000000000000000000000000000000000007c585087238000",
            "block_hash": "0x6624f87d3435cc938de6442db45e06f23582a7eeddb5ac15126d440db03e75f4",
            "block_number": 18692253,
            "transaction_hash": "0x933c80c2a18cbf64ec28662991186bd340519eb6974f3d301195b82064329fc8",
            "transaction_index": 213,
            "log_index": 512,
            "transaction_log_index": null,
            "log_type": null,
            "removed": false
            }
        );

        assert!(serde_json::from_value::<EthLog>(json).is_ok());
    }

    #[test]
    fn test_deserialize_empty_log() {
        let json = serde_json::json!({});

        assert!(serde_json::from_value::<EthLog>(json).is_err());
    }

    #[test]
    fn test_deserialize_criteria() {
        let json = serde_json::json!(
        {
            "fromBlock": 1,
            "toBlock": 2,
            "address": ["0x0000000000000000000000000000000000000001", "0x0000000000000000000000000000000000000002"],
            "topics": [
                "0x326edc3ac586176abeebb61e309fff4802231bb425463a8b205dc4c6bee35089",
                "0x24e6d6115f6446ccdee52f6150d0eb60d34dbe3072db714a11a601aeb3ee7503",
                "0x843f098f2784e8517a5ee9d050daa270e91403e1e4eaafa90bc535ef5910518c",
                "0x3ed3d15056b62bf1e6eae92a1eba5af40d8a7b42b2d05fdb94d1e3e1a61781c8"
            ]
        });
        let res = serde_json::from_value::<Criteria>(json).unwrap();
        assert_eq!(res.from, 1);
        assert_eq!(res.to, 2);
        assert_eq!(
            res.address,
            Some(vec![
                Address::from_str("0x0000000000000000000000000000000000000001").unwrap(),
                Address::from_str("0x0000000000000000000000000000000000000002").unwrap(),
            ]),
        );
        assert_eq!(
            res.topics,
            Some(vec![
                B256::from_str(
                    "0x326edc3ac586176abeebb61e309fff4802231bb425463a8b205dc4c6bee35089"
                )
                .unwrap(),
                B256::from_str(
                    "0x24e6d6115f6446ccdee52f6150d0eb60d34dbe3072db714a11a601aeb3ee7503"
                )
                .unwrap(),
                B256::from_str(
                    "0x843f098f2784e8517a5ee9d050daa270e91403e1e4eaafa90bc535ef5910518c"
                )
                .unwrap(),
                B256::from_str(
                    "0x3ed3d15056b62bf1e6eae92a1eba5af40d8a7b42b2d05fdb94d1e3e1a61781c8"
                )
                .unwrap()
            ]),
        );
    }

    #[test]
    fn test_deserialize_empty_criteria() {
        let json = serde_json::json!({});

        assert!(serde_json::from_value::<Criteria>(json).is_err());
    }
}
