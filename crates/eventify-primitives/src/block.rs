use chrono::prelude::*;
use ethers_core::types::{Bloom, Bytes, Withdrawal, H160, H256, H64, U256, U64};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

// Minimal Block
//   -> L1Block
//   -> ZksyncBlock
//   -> StarkwareBlock
//
//

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq, FromRow, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    pub author: Option<H160>,
    pub number: Option<U64>,
    pub hash: Option<H256>,
    pub parent_hash: H256,
    pub uncles_hash: H256,

    pub state_root: H256,
    pub transactions_root: H256,
    pub receipts_root: H256,

    pub gas_used: U256,
    pub gas_limit: U256,
    pub base_fee_per_gas: Option<U256>,
    pub blob_gas_used: Option<U256>,
    pub excess_blob_gas: Option<U256>,

    pub extra_data: Bytes,
    pub logs_bloom: Option<Bloom>,
    pub timestamp: U256,
    pub difficulty: U256,
    pub total_difficulty: Option<U256>,
    pub seal_fields: Vec<Bytes>,
    pub uncles: Vec<H256>,
    pub size: Option<U256>,
    pub mix_hash: Option<H256>,
    pub nonce: Option<H64>,
    // those two are entirely consensus layer related, might be worth skipping
    //pub withdrawals_root: Option<H256>,
    //pub withdrawals: Option<Vec<Withdrawal>>,
}

/*
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq, FromRow, ToSchema)]
pub struct StoredBlock {
    pub hash: Vec<u8>,
    pub parent_hash: Vec<u8>,
    pub uncles_hash: Vec<u8>,
    pub author: Vec<u8>,
    pub state_root: Vec<u8>,
    pub transaction_root: Vec<u8>,
    pub receitps_root: Vec<u8>,
    pub number: i64,
    pub gas_used: [u8; 32],
    pub gas_limit: [u8; 32],
    pub extra_data: Vec<u8>,
    pub logs_bloom: Vec<u8>,
    pub timestamp: [u8; 32],
    pub difficulty: [u8; 32],
    pub total_difficulty: [u8; 32],
    pub seal_fields: Vec<u8>,
    pub uncles: Vec<u8>,
    pub size: [u8; 32],
    pub mix_hash: Vec<u8>,
    pub nonce: Vec<u8>,
    pub base_fee_per_gas: [u8; 32],
    pub blob_gas_used: [u8; 32],
    pub excess_blob_gas: [u8; 32],
    pub withdrawals_root: Vec<u8>,
}

impl From<StoredBlock> for Block {
    fn from(block: StoredBlock) -> Block {
        Block {
            hash: Some(H256::from_slice(&block.hash)),
            parent_hash:
        }
    }
}
*/

//impl From<Block> for StoredBlock {
//    fn from(block: Block) -> StoredBlock {
//        StoredBlock {
//
//        }
//    }
//}

impl From<crate::ETHBlock<crate::ETHTransaction>> for Block {
    fn from(block: crate::ETHBlock<crate::ETHTransaction>) -> Self {
        Self {
            hash: block.hash,
            parent_hash: block.parent_hash,
            uncles_hash: block.uncles_hash,
            author: block.author,
            state_root: block.state_root,
            transactions_root: block.transactions_root,
            receipts_root: block.receipts_root,
            number: block.number,
            gas_used: block.gas_used,
            gas_limit: block.gas_limit,
            extra_data: block.extra_data,
            logs_bloom: block.logs_bloom,
            timestamp: block.timestamp,
            difficulty: block.difficulty,
            total_difficulty: block.total_difficulty,
            seal_fields: block.seal_fields,
            uncles: block.uncles,
            size: block.size,
            mix_hash: block.mix_hash,
            nonce: block.nonce,
            base_fee_per_gas: block.base_fee_per_gas,
            blob_gas_used: block.blob_gas_used,
            excess_blob_gas: block.excess_blob_gas,
            //withdrawals_root: block.withdrawals_root,
            //withdrawals: block.withdrawals,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Block;

    #[test]
    fn deserialize_block() {
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

        serde_json::from_value::<Block>(json).unwrap();
    }

    #[test]
    fn deserialize_empty_block() {
        let json = serde_json::json!({});

        assert!(serde_json::from_value::<Block>(json).is_err());
    }
}
