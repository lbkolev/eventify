use std::{
    fmt::{Display, Formatter},
    fs,
    str::FromStr,
};

use alloy_primitives::{keccak256, Address, BlockNumber, Bytes, B256, U64};
use serde::{Deserialize, Serialize};
use serde_json::Error;
use sqlx::prelude::FromRow;
use utoipa::ToSchema;

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq, FromRow, ToSchema)]
pub struct EthLog {
    pub removed: bool,
    #[serde(rename = "logIndex")]
    pub log_index: Option<U64>,
    #[serde(rename = "transactionIndex")]
    pub transaction_index: Option<U64>,
    #[serde(rename = "transactionHash")]
    pub transaction_hash: Option<B256>,
    #[serde(rename = "blockHash")]
    pub block_hash: Option<B256>,
    #[serde(rename = "blockNumber")]
    pub block_number: Option<U64>,
    pub address: Address,
    pub data: Bytes,
    pub topics: Vec<Option<B256>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Criteria {
    pub name: String,
    pub src_block: Option<BlockNumber>,
    pub dst_block: Option<BlockNumber>,
    pub addresses: Option<Vec<Address>>,

    pub filter0: Option<Vec<String>>, // aka event signature
    pub filter1: Option<Vec<String>>,
    pub filter2: Option<Vec<String>>,
    pub filter3: Option<Vec<String>>,
}

impl Criteria {
    pub fn from_file(file_path: &str) -> crate::Result<Criteria> {
        let contents = fs::read_to_string(file_path)
            .map_err(|e| crate::Error::InvalidCriteriaFile(e.to_string()))?;

        let criteria = serde_json::from_str(&contents)
            .map_err(|e| crate::Error::InvalidCriteriaFile(e.to_string()))?;

        Ok(criteria)
    }

    pub fn hashed_events(&self) -> Vec<B256> {
        self.filter0
            .clone()
            .unwrap_or_default()
            .into_iter()
            .map(|event| B256::from(keccak256(event)))
            .collect()
    }

    pub fn filter_as_b256(&self) -> Vec<B256> {
        self.filter1
            .clone()
            .unwrap_or_default()
            .into_iter()
            .map(|f| B256::from_str(&f).unwrap())
            .collect()
    }
}

impl Display for Criteria {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

impl FromStr for Criteria {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

impl From<&str> for Criteria {
    fn from(s: &str) -> Self {
        serde_json::from_str(s).expect("failed to parse criteria")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_log() {
        let json = serde_json::json!(
            {
            "address": "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2",
            "topics": [
                "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef",
                "0x000000000000000000000000a7ca2c8673bcfa5a26d8ceec2887f2cc2b0db22a",
                "0x00000000000000000000000006da0fd433c1a5d7a4faa01111c044910a184553"
            ],
            "data": "0x000000000000000000000000000000000000000000000000007c585087238000",
            "block_hash": "0x6624f87d3435cc938de6442db45e06f23582a7eeddb5ac15126d440db03e75f4",
            "block_number": 18692253,
            "transaction_hash": "0x933c80c2a18cbf64ec28662991186bd340519eb6974f3d301195b82064329fc8",
            "transaction_index": 213,
            "log_index": 512,
            "transaction_log_index": null,
            "log_type": null,
            "removed": false
            }
        );

        assert!(serde_json::from_value::<EthLog>(json).is_ok());
    }

    #[test]
    fn test_deserialize_empty_log() {
        let json = serde_json::json!({});

        assert!(serde_json::from_value::<EthLog>(json).is_err());
    }

    #[test]
    fn test_deserialize_criteria() {
        let json = serde_json::json!(
        {
            "name": "test",
            "srcBlock": "1",
            "dstBlock": "2",
            "addresses": ["0x0000000000000000000000000000000000000001", "0x0000000000000000000000000000000000000002"],
            "filter0": ["Transfer(address,address,uint256)"],
            "filter1": ["0x000000"],
            "filter2": ["0x000000"],
            "filter3": ["0x000000"]
        });
        let res = serde_json::from_value::<Criteria>(json).unwrap();
        assert_eq!(res.name, "test");
        assert_eq!(res.src_block, Some(BlockNumber::Number(U64::from(1))));
        assert_eq!(res.dst_block, Some(BlockNumber::Number(U64::from(2))));
        assert_eq!(
            res.addresses,
            Some(vec![
                Address::from_str("0x0000000000000000000000000000000000000001").unwrap(),
                Address::from_str("0x0000000000000000000000000000000000000002").unwrap(),
            ]),
        );
        assert_eq!(
            res.filter0,
            Some(vec!["Transfer(address,address,uint256)".to_string()]),
        );
        assert_eq!(res.filter1, Some(vec!["0x000000".to_string()]));
        assert_eq!(res.filter2, Some(vec!["0x000000".to_string()]));
        assert_eq!(res.filter3, Some(vec!["0x000000".to_string()]));

        let json = serde_json::json!(
        {
            "name": "test",
            "src_block": 1,
            "dst_block": 2,
        });
        assert!(serde_json::from_value::<Criteria>(json).is_ok());
    }

    #[test]
    fn test_deserialize_empty_criteria() {
        let json = serde_json::json!({});

        assert!(serde_json::from_value::<Criteria>(json).is_err());
    }
}
