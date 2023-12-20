use ethers_core::types::{Address, Bytes, H256, U256, U64};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub hash: H256,
    pub nonce: U256,
    pub block_hash: Option<H256>,
    pub block_number: Option<U64>,
    pub transaction_index: Option<U64>,
    pub from: Address,
    pub to: Option<Address>,
    pub value: U256,
    pub gas_price: Option<U256>,
    pub gas: U256,
    pub input: Bytes,
    pub v: U64,
    pub r: U256,
    pub s: U256,
    pub transaction_type: Option<U64>,
    pub max_fee_per_gas: Option<U256>,
    pub max_priority_fee_per_gas: Option<U256>,
}

impl Transaction {
    pub fn contract_creation(&self) -> bool {
        self.to.is_none()
    }
}

impl From<crate::ETHTransaction> for Transaction {
    fn from(tx: crate::ETHTransaction) -> Self {
        Self {
            hash: tx.hash,
            nonce: tx.nonce,
            block_hash: tx.block_hash,
            block_number: tx.block_number,
            transaction_index: tx.transaction_index,
            from: tx.from,
            to: tx.to,
            value: tx.value,
            gas_price: tx.gas_price,
            gas: tx.gas,
            input: tx.input,
            v: tx.v,
            r: tx.r,
            s: tx.s,
            transaction_type: tx.transaction_type,
            max_fee_per_gas: tx.max_fee_per_gas,
            max_priority_fee_per_gas: tx.max_priority_fee_per_gas,
        }
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

        serde_json::from_value::<Transaction>(json).unwrap();
    }

    #[test]
    fn deserialize_empty_tx() {
        let json = serde_json::json!({});

        assert!(serde_json::from_value::<Transaction>(json).is_err());
    }

    #[test]
    fn test_is_contract_creation() {
        let tx = Transaction {
            to: None,
            ..Default::default() // Using other default values
        };

        assert!(tx.contract_creation());
    }
}
