use std::{fmt::Debug, hash::Hash};

use alloy_primitives::{B256, U256};
use eyre::Result;
use redis::AsyncCommands;
use sqlx::{Error as SqlError, FromRow};
use utoipa::ToSchema;

use crate::{
    networks::{core::CoreBlock, NetworkKind, ResourceKind},
    traits::{Block, Emit, Insert},
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
pub struct EthBlock {
    #[serde(flatten)]
    core: CoreBlock,

    #[serde(rename = "withdrawalsRoot")]
    pub withdrawals_hash: Option<B256>,

    #[serde(rename = "totalDifficulty")]
    pub total_difficulty: Option<U256>,

    /// added by EIP-1559
    #[serde(rename = "baseFeePerGas")]
    pub base_fee: Option<U256>,

    /// added by EIP-4844
    #[serde(rename = "blobGasUsed")]
    pub blob_gas_used: Option<U256>,

    /// added by EIP-4844
    #[serde(rename = "excessBlobGas")]
    pub excess_blob_gas: Option<U256>,

    /// added by EIP-4788
    #[serde(rename = "parentBeaconBlockRoot")]
    pub parent_beacon_root: Option<B256>,
}

impl Block for EthBlock {
    fn core(&self) -> &CoreBlock {
        &self.core
    }
}

impl Insert for EthBlock {
    async fn insert(&self, pool: &sqlx::PgPool, _: &Option<B256>) -> Result<(), SqlError> {
        let (
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
        ) = self.core().db_repr();

        let withdrawals_hash = self.withdrawals_hash.as_ref().map(|v| v.as_slice());
        let total_difficulty = self.total_difficulty.as_ref().map(|v| v.as_le_slice());
        let base_fee = self.base_fee.map(|v| v.to::<i64>());
        let parent_beacon_root = self.parent_beacon_root.as_ref().map(|v| v.as_slice());
        let blob_gas_used = self.blob_gas_used.as_ref().map(|v| v.as_le_slice());
        let excess_blob_gas = self.excess_blob_gas.as_ref().map(|v| v.as_le_slice());

        let sql = r#"INSERT INTO block (
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
            time,

            withdrawals_hash,
            total_difficulty,
            base_fee,
            parent_beacon_root,
            blob_gas_used,
            excess_blob_gas
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23
            ) ON CONFLICT DO NOTHING"#;

        sqlx::query(sql)
            .bind(NetworkKind::Ethereum)
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
            .bind(withdrawals_hash)
            .bind(total_difficulty)
            .bind(base_fee)
            .bind(parent_beacon_root)
            .bind(blob_gas_used)
            .bind(excess_blob_gas)
            .execute(pool)
            .await?;

        Ok(())
    }
}

impl Emit for EthBlock {
    async fn emit(
        &self,
        queue: &redis::Client,
        network: &crate::networks::NetworkKind,
    ) -> Result<(), EmitError> {
        let mut con = queue.get_async_connection().await?;

        let channel = format!("{}:{}", network, ResourceKind::Block);
        con.lpush(channel, serde_json::to_string(self)?).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_eth_block() {
        let json = serde_json::json!(
            {
                "parentHash": "0xe21d9fc49e447805ab1f8cf3c647aa12bf8342c4418076ee9ef2e9fb8d551136",
                "sha3Uncles": "0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347",
                "miner": "0xe43cc5b6ff052f5aa931a4f9ef2bfa0c500014ca",
                "stateRoot": "0x06d72f7ea43994c1ecc8c639367751c60c314efd8ad7d1ff45683b5db841ba09",
                "transactionsRoot": "0x93069aa747ff207ed717eb530f1a297104473fb71cfa5422b6300581c656e02c",
                "receiptsRoot": "0xb2e099561441fa9d7d25d69780d185eb00f5ca39033679ef827423ddc68b81f0",
                "logsBloom": "0x2d3110858080012b80550082c0a412160001000064020408800522102ac019100040140842a8804090412300100c45040a61022008402c0702429080042b050128004288012088084c10180880d870a004008205e17001809489014000ec088005000080229009690200d100801808040320906104284e0c0601e450044a0004800091c0816a100020400242000a00c041600020210050280002a35b11900811c700202000822120001050805138008045400080240298081020042252a08c50130c000601930100100e00810286a0448260000205200018e8d200824010f00030f03001010010060000766328406081082a15624211406504a0089001c006a0",
                "difficulty": "0x0",
                "number": "0x128669b",
                "gasLimit": "0x1c9c380",
                "gasUsed": "0x377830",
                "timestamp": "0x65f161eb",
                "extraData": "0xd883010d0e846765746888676f312e32312e36856c696e7578",
                "mixHash": "0xc282f24a1ac767946aafb8de743847a3561b4f0dc203e6e0093660670a77ffdc",
                "nonce": "0x0000000000000000",
                "baseFeePerGas": "0xb00d096a1",
                "withdrawalsRoot": "0x4be4c436558be298a46793081f03ea74aaedd8fb5ac8ee90ab1eba42b1a38f35",
                "blobGasUsed": null,
                "excessBlobGas": null,
                "parentBeaconBlockRoot": null,
                "hash": "0xe2eb1899da1f3c73105cfd383de9f7792c9491a60d5ac1a61a68c521c0c53902"
              }
        );

        assert!(serde_json::from_value::<EthBlock>(json).is_ok());
    }

    #[test]
    fn deserialize_empty_eth_block() {
        let json = serde_json::json!({});

        assert!(serde_json::from_value::<EthBlock>(json).is_err());
    }
}
