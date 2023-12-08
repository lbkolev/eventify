use ethers_core::types::{Address, Bytes, Transaction, H256, U256, U64};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct IndexedTransaction(Transaction);

impl From<Transaction> for IndexedTransaction {
    fn from(tx: Transaction) -> Self {
        Self(tx)
    }
}

impl IndexedTransaction {
    pub fn hash(&self) -> H256 {
        self.0.hash
    }

    pub fn nonce(&self) -> U256 {
        self.0.nonce
    }

    pub fn block_hash(&self) -> Option<H256> {
        self.0.block_hash
    }

    pub fn block_number(&self) -> Option<U64> {
        self.0.block_number
    }

    pub fn transaction_index(&self) -> Option<U64> {
        self.0.transaction_index
    }

    pub fn _from(&self) -> Address {
        self.0.from
    }

    pub fn to(&self) -> Option<Address> {
        self.0.to
    }

    pub fn value(&self) -> U256 {
        self.0.value
    }

    pub fn gas_price(&self) -> Option<U256> {
        self.0.gas_price
    }

    pub fn gas(&self) -> U256 {
        self.0.gas
    }

    pub fn input(&self) -> &Bytes {
        &self.0.input
    }

    pub fn v(&self) -> U64 {
        self.0.v
    }

    pub fn r(&self) -> U256 {
        self.0.r
    }

    pub fn s(&self) -> U256 {
        self.0.s
    }

    pub fn transaction_type(&self) -> Option<U64> {
        self.0.transaction_type
    }

    pub fn max_fee_per_gas(&self) -> Option<U256> {
        self.0.max_fee_per_gas
    }

    pub fn max_priority_fee_per_gas(&self) -> Option<U256> {
        self.0.max_priority_fee_per_gas
    }

    /// Determines if the transaction is creating a contract.
    pub fn contract_creation(&self) -> bool {
        self.0.to.is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_tx() {
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

        serde_json::from_value::<IndexedTransaction>(json).unwrap();
    }
}
