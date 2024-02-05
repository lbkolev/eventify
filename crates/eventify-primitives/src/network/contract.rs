use alloy_primitives::{Address, Bytes, B256};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;

use super::transaction::EthTransaction;

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
#[serde(rename_all = "camelCase")]
pub struct Contract {
    pub transaction_hash: B256,
    pub from: Address,
    pub input: Bytes,
}

impl From<EthTransaction> for Contract {
    fn from(tx: EthTransaction) -> Self {
        Self {
            transaction_hash: tx.hash,
            from: tx.from,
            input: tx.input,
        }
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
