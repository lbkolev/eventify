use serde::{Deserialize, Serialize};
use web3::types::{Bytes, Transaction, H160, H256, U256, U64};

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
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

    pub fn _from(&self) -> Option<H160> {
        self.0.from
    }

    pub fn to(&self) -> Option<H160> {
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

    pub fn v(&self) -> Option<U64> {
        self.0.v
    }

    pub fn r(&self) -> Option<U256> {
        self.0.r
    }

    pub fn s(&self) -> Option<U256> {
        self.0.s
    }

    pub fn raw(&self) -> &Option<Bytes> {
        &self.0.raw
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

    /// Determines if the transaction is a contract creation one.
    pub fn contract_creation(&self) -> bool {
        self.0.to.is_none()
    }

    // Determines if the transaction is considered special.
    //
    // Special transactions are the ones that are indexed into their own tables.
    //fn special(&self) -> bool {
    //    self.input.0.len() >= 4
    //        && (&self.input.0[0..4] == ERC20_APPROVE_SIGNATURE
    //            || &self.input.0[0..4] == ERC20_TRANSFER_FROM_SIGNATURE
    //            || &self.input.0[0..4] == ERC20_TRANSFER_SIGNATURE
    //            || &self.input.0[0..4] == ERC721_SAFE_TRANSFER_FROM_SIGNATURE)
    //}

    ///// Processes the transaction.
    /////
    ///// If the transaction is considered special, it's indexed into its own table.
    ///// If the transaction is not considered special, but we've got a function signature that matches the transaction's input, it is indexed into the `transaction` table.
    ////pub async fn process<T: Storage>(&self, conn: &T) -> Result<()> {
    //    if self.special() {
    //        match &self.input.0[0..4] {
    //            ERC20_APPROVE_SIGNATURE => {
    //                conn.insert_approve(&Approve::try_from(self.clone())?).await
    //            }
    //            ERC20_TRANSFER_FROM_SIGNATURE => {
    //                conn.insert_transfer_from(&TransferFrom::try_from(self.clone())?)
    //                    .await
    //            }
    //            ERC20_TRANSFER_SIGNATURE => {
    //                conn.insert_transfer(&Transfer::try_from(self.clone())?)
    //                    .await
    //            }
    //            ERC721_SAFE_TRANSFER_FROM_SIGNATURE => {
    //                log::warn!("ERC721 safe transfer from is not implemented yet");
    //                Ok(())
    //            }
    //            _ => unreachable!(),
    //        }
    //    } else {
    //        log::debug!("Transaction {:?} is not considered special", self.hash);
    //        Ok(())
    //    }
    //}
}

#[derive(
    derive_builder::Builder, Clone, Debug, Default, serde::Deserialize, serde::Serialize, PartialEq,
)]
#[serde(rename_all = "camelCase")]
pub(super) struct TransactionBoilerplate {
    pub contract_addr: H160,
    pub transaction_hash: H256,
    pub transaction_sender: H160,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_transaction() {
        let json = serde_json::json!({
            "hash": "0x422fb0d5953c0c48cbb42fb58e1c30f5e150441c68374d70ca7d4f191fd56f26",
            "from": "0x407d73d8a49eeb85d32cf465507dd71d507100c1",
            "to": "0x853f43d8a49eeb85d32cf465507dd71d507100c1",
            "input": "0x603880600c6000396000f300603880600c6000396000f3603880600c6000396000f360",
        });

        serde_json::from_value::<IndexedTransaction>(json).unwrap();
    }

    #[test]
    fn serialize_half_empty_transaction() {
        let json = serde_json::json!({
            "hash": "0x422fb0d5953c0c48cbb42fb58e1c30f5e150441c68374d70ca7d4f191fd56f26",
            "from": None::<H160>,
            "to": None::<H160>,
            "input": "0x603880600c6000396000f300603880600c6000396000f3603880600c6000396000f360",
        });

        serde_json::from_value::<IndexedTransaction>(json).unwrap();
    }

    #[test]
    fn serialize_transaction_boilerplate() {
        let json = serde_json::json!({
            "contractAddr": "0x407d73d8a49eeb85d32cf465507dd71d507100c1",
            "transactionHash": "0x422fb0d5953c0c48cbb42fb58e1c30f5e150441c68374d70ca7d4f191fd56f26",
            "transactionSender": "0x407d73d8a49eeb85d32cf465507dd71d507100c1",
        });

        serde_json::from_value::<TransactionBoilerplate>(json).unwrap();
    }
}
