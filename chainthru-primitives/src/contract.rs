use ethers_core::types::{Bytes, H160, H256};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Contract {
    pub address: H160,
    pub transaction_hash: H256,
    pub from: H160,
    pub input: Bytes,
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
