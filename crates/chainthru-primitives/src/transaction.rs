use ethers_core::types::{Address, Bytes, Transaction, H256, U256, U64};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
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

    pub fn input(&self) -> Bytes {
        self.0.input.clone()
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

    //#[test]
    //fn serialize_transaction() {
    //    let json = serde_json::json!({
    //        "hash": "0x422fb0d5953c0c48cbb42fb58e1c30f5e150441c68374d70ca7d4f191fd56f26",
    //        "from": "0x407d73d8a49eeb85d32cf465507dd71d507100c1",
    //        "to": "0x853f43d8a49eeb85d32cf465507dd71d507100c1",
    //        "input": "0x603880600c6000396000f300603880600c6000396000f3603880600c6000396000f360",
    //    });

    //    serde_json::from_value::<IndexedTransaction>(json).unwrap();
    //}

    //#[test]
    //fn serialize_half_empty_transaction() {
    //    let json = serde_json::json!({
    //        "hash": "0x422fb0d5953c0c48cbb42fb58e1c30f5e150441c68374d70ca7d4f191fd56f26",
    //        "from": None::<H160>,
    //        "to": None::<H160>,
    //        "input": "0x603880600c6000396000f300603880600c6000396000f3603880600c6000396000f360",
    //    });

    //    serde_json::from_value::<IndexedTransaction>(json).unwrap();
    //}

    //#[test]
    //fn serialize_transaction_boilerplate() {
    //    let json = serde_json::json!({
    //        "contractAddr": "0x407d73d8a49eeb85d32cf465507dd71d507100c1",
    //        "transactionHash": "0x422fb0d5953c0c48cbb42fb58e1c30f5e150441c68374d70ca7d4f191fd56f26",
    //        "transactionSender": "0x407d73d8a49eeb85d32cf465507dd71d507100c1",
    //    });

    //    serde_json::from_value::<TransactionBoilerplate>(json).unwrap();
    //}
}
