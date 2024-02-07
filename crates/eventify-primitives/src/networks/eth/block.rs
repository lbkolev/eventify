use alloy_primitives::{Address, Bloom, Bytes, B256, B64, U256, U64};
use sqlx::FromRow;
use utoipa::ToSchema;

use super::EthTransaction;
use crate::traits::Block;

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
    #[serde(rename = "blobGasUsed")]
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

impl Block for EthBlock<alloy_primitives::B256> {
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
