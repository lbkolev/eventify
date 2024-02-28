use alloy_primitives::{Address, Bloom, Bytes, B256, B64, U256, U64};
use eyre::Result;
use redis::{Commands, RedisError};
use sqlx::{Error, FromRow};
use utoipa::ToSchema;

use super::EthTransaction;
use crate::{
    networks::ResourceKind,
    traits::{Block, Emit, Insert},
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
pub struct EthBlock<T> {
    // -- header
    #[serde(rename = "parentHash")]
    pub parent_hash: B256,
    #[serde(rename = "sha3Uncles")]
    pub uncle_hash: B256,
    #[serde(rename = "miner")]
    pub coinbase: Address,
    #[serde(rename = "stateRoot")]
    pub root: B256,
    #[serde(rename = "transactionsRoot")]
    pub tx_hash: B256,
    #[serde(rename = "receiptsRoot")]
    pub receipt_hash: B256,
    #[serde(rename = "logsBloom")]
    pub bloom: Option<Bloom>,
    pub difficulty: U256,
    pub number: Option<U64>,
    #[serde(rename = "gasLimit")]
    pub gas_limit: U256,
    #[serde(rename = "gasUsed")]
    pub gas_used: U256,
    #[serde(rename = "timestamp")]
    pub time: U256,
    #[serde(rename = "extraData")]
    pub extra: Bytes,
    #[serde(rename = "mixHash")]
    pub mix_digest: B256,
    pub nonce: Option<B64>,

    /// added by EIP-1559
    #[serde(rename = "baseFeePerGas")]
    pub base_fee: Option<U256>,

    /// added by EIP-4788
    #[serde(rename = "parentBeaconBlockRoot")]
    pub parent_beacon_root: Option<B256>,

    /// added by EIP-4844
    #[serde(rename = "blobGasUsed")]
    pub blob_gas_used: Option<U256>,

    /// added by EIP-4844
    #[serde(rename = "excessBlobGas")]
    pub excess_blob_gas: Option<U256>,

    /// added by EIP-4895
    #[serde(rename = "withdrawalsHash")]
    pub withdrawals_hash: Option<B256>,
    // --

    // -- body
    // either list of tx hashes or list of tx objects
    pub transactions: Option<Vec<T>>,
    pub hash: Option<B256>,
    // --
}

impl Block for EthBlock<B256> {
    fn parent_hash(&self) -> alloy_primitives::B256 {
        self.parent_hash
    }

    fn hash(&self) -> Option<alloy_primitives::B256> {
        self.hash
    }

    fn number(&self) -> Option<alloy_primitives::U64> {
        self.number
    }
}

impl Block for EthBlock<EthTransaction> {
    fn parent_hash(&self) -> alloy_primitives::B256 {
        self.parent_hash
    }

    fn hash(&self) -> Option<alloy_primitives::B256> {
        self.hash
    }

    fn number(&self) -> Option<alloy_primitives::U64> {
        self.number
    }
}

impl<T: Send + Sync> Insert for EthBlock<T> {
    async fn insert(
        &self,
        pool: &sqlx::PgPool,
        schema: &str,
        _: &Option<B256>,
    ) -> Result<(), Error> {
        let parent_hash = self.parent_hash.as_slice();
        let uncle_hash = self.uncle_hash.as_slice();
        let coinbase = self.coinbase.as_slice();
        let root = self.root.as_slice();
        let tx_hash = self.tx_hash.as_slice();
        let receipt_hash = self.receipt_hash.as_slice();
        let difficulty = self.difficulty.as_le_slice();
        let number = self.number.map(|v| v.to::<i64>());
        let gas_limit = self.gas_limit.as_le_slice();
        let gas_used = self.gas_used.as_le_slice();
        let time = self.time.to::<i64>();
        let extra = self.extra.to_vec();
        let mix_digest = self.mix_digest.as_slice();
        let nonce = self.nonce.as_ref().map(|v| v.as_slice());
        let base_fee = self.base_fee.map(|v| v.to::<i64>());
        let parent_beacon_root = self.parent_beacon_root.as_ref().map(|v| v.as_slice());
        let blob_gas_used = self.blob_gas_used.map(|v| v.to::<i64>());
        let excess_blob_gas = self.excess_blob_gas.map(|v| v.to::<i64>());
        let withdrawals_hash = self.withdrawals_hash.as_ref().map(|v| v.as_slice());
        let hash = self.hash.as_ref().map(|v| v.as_slice());

        let sql = format!(
            r#"INSERT INTO {schema}.block (
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
                ) ON CONFLICT DO NOTHING"#,
        );

        sqlx::query(&sql)
            .bind(parent_hash)
            .bind(uncle_hash)
            .bind(coinbase)
            .bind(root)
            .bind(tx_hash)
            .bind(receipt_hash)
            .bind(difficulty)
            .bind(number)
            .bind(gas_limit)
            .bind(gas_used)
            .bind(time)
            .bind(extra)
            .bind(mix_digest)
            .bind(nonce)
            .bind(base_fee)
            .bind(parent_beacon_root)
            .bind(blob_gas_used)
            .bind(excess_blob_gas)
            .bind(withdrawals_hash)
            .bind(hash)
            .execute(pool)
            .await?;

        Ok(())
    }
}

impl<B: Send + Sync> Emit for EthBlock<B> {
    async fn emit<T: serde::Serialize + Send + Sync>(
        &self,
        queue: &redis::Client,
        network: &crate::networks::NetworkKind,
        message: &T,
    ) -> Result<(), RedisError> {
        let mut con = queue.get_connection()?;
        let channel = format!("{}:{}", network, ResourceKind::Block);

        con.lpush(channel, serde_json::to_string(message).unwrap())?;
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

        serde_json::from_value::<EthBlock<B256>>(json).unwrap();
    }

    #[test]
    fn deserialize_empty_eth_block() {
        let json = serde_json::json!({});

        assert!(serde_json::from_value::<EthBlock<B256>>(json).is_err());
    }
}
