use std::{fmt::Debug, hash::Hash};

use alloy_primitives::{Address, Bytes, B256, B64, U256, U64};
use eyre::Result;
use redis::AsyncCommands;
use sqlx::{Error as SqlError, FromRow};
use utoipa::ToSchema;

use crate::{
    networks::{LogKind, NetworkKind, ResourceKind},
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
pub struct CoreBlock {
    pub number: Option<U64>,
    pub hash: Option<B256>,
    #[serde(rename = "parentHash")]
    pub parent_hash: B256,
    #[serde(rename = "mixHash")]
    pub mix_digest: Option<B256>,
    #[serde(rename = "sha3Uncles")]
    pub uncle_hash: B256,
    #[serde(rename = "receiptsRoot")]
    pub receipt_hash: B256,
    #[serde(rename = "stateRoot")]
    pub root: B256,
    #[serde(rename = "transactionsRoot")]
    pub tx_hash: B256,
    #[serde(rename = "miner")]
    pub coinbase: Address,
    pub nonce: Option<B64>,
    #[serde(rename = "gasUsed")]
    pub gas_used: U256,
    #[serde(rename = "gasLimit")]
    pub gas_limit: U256,
    pub difficulty: U256,
    #[serde(rename = "extraData")]
    pub extra: Bytes,
    #[serde(rename = "logsBloom")]
    pub bloom: Option<Bytes>,
    #[serde(rename = "timestamp")]
    pub time: U256,
}

impl CoreBlock {
    pub fn number(&self) -> Option<U64> {
        self.number
    }

    pub fn hash(&self) -> Option<B256> {
        self.hash
    }

    pub fn parent_hash(&self) -> B256 {
        self.parent_hash
    }

    pub fn mix_digest(&self) -> Option<B256> {
        self.mix_digest
    }

    pub fn uncle_hash(&self) -> B256 {
        self.uncle_hash
    }

    pub fn receipt_hash(&self) -> B256 {
        self.receipt_hash
    }

    pub fn root(&self) -> B256 {
        self.root
    }

    pub fn tx_hash(&self) -> B256 {
        self.tx_hash
    }

    pub fn coinbase(&self) -> Address {
        self.coinbase
    }

    pub fn nonce(&self) -> Option<B64> {
        self.nonce
    }

    pub fn gas_used(&self) -> U256 {
        self.gas_used
    }

    pub fn gas_limit(&self) -> U256 {
        self.gas_limit
    }

    pub fn difficulty(&self) -> U256 {
        self.difficulty
    }

    pub fn extra(&self) -> Bytes {
        self.extra.clone()
    }

    pub fn bloom(&self) -> Option<Bytes> {
        self.bloom.clone()
    }

    pub fn time(&self) -> U256 {
        self.time
    }

    pub async fn insert(&self, pool: &sqlx::PgPool, network: NetworkKind) -> Result<(), SqlError> {
        let number = self.number.map(|v| v.to::<i64>());
        let hash = self.hash.as_ref().map(|v| v.as_slice());
        let parent_hash = self.parent_hash.as_slice();
        let mix_digest = self.mix_digest.as_ref().map(|v| v.as_slice());
        let uncle_hash = self.uncle_hash.as_slice();
        let receipt_hash = self.receipt_hash.as_slice();
        let root = self.root.as_slice();
        let tx_hash = self.tx_hash.as_slice();
        let coinbase = self.coinbase.as_slice();
        let nonce = self.nonce.as_ref().map(|v| v.as_slice());
        let gas_used = self.gas_used.as_le_slice();
        let gas_limit = self.gas_limit.as_le_slice();
        let difficulty = self.difficulty.as_le_slice();
        let extra = self.extra.to_vec();
        let bloom = self.bloom.as_ref().map(|v| v.to_vec());
        let time = self.time.to::<i64>();

        let query = r#"
            INSERT INTO block (
                network,

                number,
                hash,
                parent_hash,
                mix_digest,
                uncle_hash,
                receipt_hash,
                root,
                tx_hash,
                coinbase,
                nonce,
                gas_used,
                gas_limit,
                difficulty,
                extra,
                bloom,
                time
            )
            VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17
            ) ON CONFLICT DO NOTHING
        "#;

        sqlx::query(query)
            .bind(network)
            .bind(number)
            .bind(hash)
            .bind(parent_hash)
            .bind(mix_digest)
            .bind(uncle_hash)
            .bind(receipt_hash)
            .bind(root)
            .bind(tx_hash)
            .bind(coinbase)
            .bind(nonce)
            .bind(gas_used)
            .bind(gas_limit)
            .bind(difficulty)
            .bind(extra)
            .bind(bloom)
            .bind(time)
            .execute(pool)
            .await?;

        Ok(())
    }

    pub async fn emit(
        &self,
        queue: &redis::Client,
        network: &NetworkKind,
    ) -> Result<(), EmitError> {
        let mut con = queue.get_async_connection().await?;

        let channel = format!("{}:{}", network, ResourceKind::Block);
        con.lpush(channel, serde_json::to_string(self)?).await?;

        Ok(())
    }

    #[allow(clippy::type_complexity)]
    pub(crate) fn db_repr(
        &self,
    ) -> (
        Option<i64>,
        Option<&[u8]>,
        &[u8],
        Option<&[u8]>,
        &[u8],
        &[u8],
        &[u8],
        &[u8],
        &[u8],
        Option<&[u8]>,
        &[u8],
        &[u8],
        &[u8],
        Vec<u8>,
        Option<Vec<u8>>,
        i64,
    ) {
        let number = self.number.map(|v| v.to::<i64>());
        let hash = self.hash.as_ref().map(|v| v.as_slice());
        let parent_hash = self.parent_hash.as_slice();
        let mix_digest = self.mix_digest.as_ref().map(|v| v.as_slice());
        let uncle_hash = self.uncle_hash.as_slice();
        let receipt_hash = self.receipt_hash.as_slice();
        let root = self.root.as_slice();
        let tx_hash = self.tx_hash.as_slice();
        let coinbase = self.coinbase.as_slice();
        let nonce = self.nonce.as_ref().map(|v| v.as_slice());
        let gas_used = self.gas_used.as_le_slice();
        let gas_limit = self.gas_limit.as_le_slice();
        let difficulty = self.difficulty.as_le_slice();
        let extra = self.extra.to_vec();
        let bloom = self.bloom.as_ref().map(|v| v.to_vec());
        let time = self.time.to::<i64>();

        (
            number,
            hash,
            parent_hash,
            mix_digest,
            uncle_hash,
            receipt_hash,
            root,
            tx_hash,
            coinbase,
            nonce,
            gas_used,
            gas_limit,
            difficulty,
            extra,
            bloom,
            time,
        )
    }
}

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
pub struct CoreLog {
    pub address: Address,
    #[serde(rename = "blockHash")]
    pub block_hash: Option<B256>,
    #[serde(rename = "blockNumber")]
    pub block_number: Option<U64>,
    pub data: Bytes,
    #[serde(rename = "logIndex")]
    pub log_index: Option<U64>,
    pub removed: bool,
    pub topics: Vec<B256>,
    #[serde(rename = "transactionIndex")]
    pub tx_index: Option<U64>,
    #[serde(rename = "transactionHash")]
    pub tx_hash: Option<B256>,
}

impl CoreLog {
    pub async fn insert(&self, pool: &sqlx::PgPool, network: NetworkKind) -> Result<(), SqlError> {
        let address = self.address.as_slice();
        let block_hash = self.block_hash.as_ref().map(|v| v.as_slice());
        let block_number = self.block_number.map(|v| v.to::<i64>());
        let data = self.data.0.as_ref();
        let log_index = self.log_index.map(|v| v.to::<i64>());
        let removed = self.removed;
        let topic0 = self.topics.first().map(|v| v.as_slice());
        let topic1 = self.topics.get(1).map(|v| v.as_slice());
        let topic2 = self.topics.get(2).map(|v| v.as_slice());
        let topic3 = self.topics.get(3).map(|v| v.as_slice());
        let tx_index = self.tx_index.map(|v| v.to::<i64>());
        let tx_hash = self.tx_hash.as_ref().map(|v| v.as_slice());

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
                tx_hash
            )
            VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13
            ) ON CONFLICT DO NOTHING
        "#;

        sqlx::query(query)
            .bind(network)
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
            .execute(pool)
            .await?;

        Ok(())
    }

    pub async fn emit(
        &self,
        queue: &redis::Client,
        network: &NetworkKind,
    ) -> Result<(), EmitError> {
        let mut con = queue.get_async_connection().await?;

        let channel = format!("{}:{}", network, ResourceKind::Log(LogKind::Raw));
        con.lpush(channel, serde_json::to_string(self)?).await?;

        Ok(())
    }

    #[allow(clippy::type_complexity)]
    pub(crate) fn db_repr(
        &self,
    ) -> (
        &[u8],
        Option<&[u8]>,
        Option<i64>,
        &[u8],
        Option<i64>,
        bool,
        Option<&[u8]>,
        Option<&[u8]>,
        Option<&[u8]>,
        Option<&[u8]>,
        Option<i64>,
        Option<&[u8]>,
    ) {
        let address = self.address.as_slice();
        let block_hash = self.block_hash.as_ref().map(|v| v.as_slice());
        let block_number = self.block_number.map(|v| v.to::<i64>());
        let data = self.data.0.as_ref();
        let log_index = self.log_index.map(|v| v.to::<i64>());
        let removed = self.removed;
        let topic0 = self.topics.first().map(|v| v.as_slice());
        let topic1 = self.topics.get(1).map(|v| v.as_slice());
        let topic2 = self.topics.get(2).map(|v| v.as_slice());
        let topic3 = self.topics.get(3).map(|v| v.as_slice());
        let tx_index = self.tx_index.map(|v| v.to::<i64>());
        let tx_hash = self.tx_hash.as_ref().map(|v| v.as_slice());

        (
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
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_core_block() {
        let json = serde_json::json!({
            "hash": "0x4debecd96c87bd9be70b2a428d1e2d537e7f3ce77e353a7f031b4b66fb4d12eb",
            "parentHash": "0x0000000000000000000000000000000000000000000000000000000000000000",
            "sha3Uncles": "0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347",
            "miner": "0x0000000000000000000000000000000000000000",
            "stateRoot": "0x0000000000000000000000000000000000000000000000000000000000000000",
            "transactionsRoot": "0x0000000000000000000000000000000000000000000000000000000000000000",
            "receiptsRoot": "0x0000000000000000000000000000000000000000000000000000000000000000",
            "number": "0x1155a9",
            "gasUsed": "0x0",
            "gasLimit": "0x0",
            "extraData": "0x",
            "logsBloom": "0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
            "timestamp": "0x65f06454",
            "difficulty": "0x0",
            "mixHash": null,
            "nonce": null
        });

        assert!(serde_json::from_value::<CoreBlock>(json).is_ok());
        assert!(serde_json::from_value::<CoreBlock>(serde_json::json!({})).is_err());
    }

    #[test]
    fn deserialize_core_log() {
        let json = serde_json::json!(
            {
                "address": "0xdac17f958d2ee523a2206206994597c13d831ec7",
                "blockHash": "0xfad3e899227b47062b71c90e61eeb056a43052be544bc006031b10df8abc92f4",
                "blockNumber": "0x1286817",
                "data": "0x0000000000000000000000000000000000000000000000000000000077359400",
                "logIndex": "0x75",
                "removed": false,
                "topics": [
                  "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef",
                  "0x0000000000000000000000004d06a4779ae0ed965598a1ef2b86b95a41ad7e81",
                  "0x00000000000000000000000011235534a66a33c366b84933d5202c841539d1c9"
                ],
                "transactionHash": "0x8acd636a4e0a0165bfbf003aa202a87b1a8e17e05183650ad39415861555aa6e",
                "transactionIndex": "0x82"
              }
        );

        assert!(serde_json::from_value::<CoreLog>(json).is_ok());
        assert!(serde_json::from_value::<CoreLog>(serde_json::json!({})).is_err());
    }
}
