use ethers_core::types::{Bytes, Transaction, H160, H256};
use serde::{Deserialize, Serialize};

use crate::IndexedTransaction;

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Contract {
    pub transaction_hash: H256,
    pub _from: H160,
    pub input: Bytes,
}

impl Contract {
    pub fn transaction_hash(&self) -> H256 {
        self.transaction_hash
    }

    pub fn _from(&self) -> H160 {
        self._from
    }

    pub fn input(&self) -> &Bytes {
        &self.input
    }
}

impl From<IndexedTransaction> for Contract {
    fn from(transaction: IndexedTransaction) -> Self {
        Self {
            transaction_hash: transaction.hash(),
            _from: transaction._from(),
            input: transaction.input().clone(),
        }
    }
}

impl From<Transaction> for Contract {
    fn from(tx: Transaction) -> Self {
        let tx = IndexedTransaction::from(tx);

        Self::from(tx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_contract() {
        let json = serde_json::json!({
            "address": "0x407d73d8a49eeb85d32cf465507dd71d507100c1",
            "transactionHash": "0x422fb0d5953c0c48cbb42fb58e1c30f5e150441c68374d70ca7d4f191fd56f26",
            "from": "0x407d73d8a49eeb85d32cf465507dd71d507100c1",
            "input": "0x603880600c6000396000f300603880600c6000396000f3603880600c6000396000f360",
        });

        serde_json::from_value::<Contract>(json).unwrap();
    }
}
