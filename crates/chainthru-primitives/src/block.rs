use ethers_core::types::{
    Block, Bloom, Bytes, Transaction, Withdrawal, H160, H256, H64, U256, U64,
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct IndexedBlock(Block<Transaction>);

impl From<Block<Transaction>> for IndexedBlock {
    fn from(block: Block<Transaction>) -> Self {
        Self(block)
    }
}

impl IndexedBlock {
    pub fn hash(&self) -> Option<H256> {
        self.0.hash
    }

    pub fn parent_hash(&self) -> H256 {
        self.0.parent_hash
    }

    pub fn uncles_hash(&self) -> H256 {
        self.0.uncles_hash
    }

    pub fn author(&self) -> Option<H160> {
        self.0.author
    }

    pub fn state_root(&self) -> H256 {
        self.0.state_root
    }

    pub fn transactions_root(&self) -> H256 {
        self.0.transactions_root
    }

    pub fn receipts_root(&self) -> H256 {
        self.0.receipts_root
    }

    pub fn number(&self) -> Option<U64> {
        self.0.number
    }

    pub fn gas_used(&self) -> U256 {
        self.0.gas_used
    }

    pub fn gas_limit(&self) -> U256 {
        self.0.gas_limit
    }

    pub fn extra_data(&self) -> Bytes {
        self.0.extra_data.clone()
    }

    pub fn logs_bloom(&self) -> Option<Bloom> {
        self.0.logs_bloom
    }

    pub fn timestamp(&self) -> U256 {
        self.0.timestamp
    }

    pub fn difficulty(&self) -> U256 {
        self.0.difficulty
    }

    pub fn total_difficulty(&self) -> Option<U256> {
        self.0.total_difficulty
    }

    pub fn seal_fields(&self) -> Vec<Bytes> {
        self.0.seal_fields.clone()
    }

    pub fn uncles(&self) -> Vec<H256> {
        self.0.uncles.clone()
    }

    pub fn size(&self) -> Option<U256> {
        self.0.size
    }

    pub fn mix_hash(&self) -> Option<H256> {
        self.0.mix_hash
    }

    pub fn nonce(&self) -> Option<H64> {
        self.0.nonce
    }

    pub fn base_fee_per_gas(&self) -> Option<U256> {
        self.0.base_fee_per_gas
    }

    pub fn blob_gas_used(&self) -> Option<U256> {
        self.0.blob_gas_used
    }

    pub fn excess_blob_gas(&self) -> Option<U256> {
        self.0.excess_blob_gas
    }

    pub fn withdrawals_root(&self) -> Option<H256> {
        self.0.withdrawals_root
    }

    pub fn withdrawals(&self) -> Option<Vec<Withdrawal>> {
        self.0.withdrawals.clone()
    }
}

#[cfg(test)]
mod tests {
    use serde_json::value::Index;

    use super::IndexedBlock;

    #[test]
    fn deserialize_block() {
        let json = serde_json::json!(
        {
            "baseFeePerGas": "0x7",
            "miner": "0x0000000000000000000000000000000000000001",
            "number": "0x1b4",
            "hash": "0x0e670ec64341771606e55d6b4ca35a1a6b75ee3d5145a99d05921026d1527331",
            "parentHash": "0x9646252be9520f6e71339a8df9c55e4d7619deeb018d2a3f2d21fc165dde5eb5",
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

        serde_json::from_value::<IndexedBlock>(json).unwrap();
    }

    #[test]
    fn deserialize_empty_block() {
        let json = serde_json::json!({});

        serde_json::from_value::<IndexedBlock>(json).unwrap();
    }
}
