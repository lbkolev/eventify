use std::{
    fmt::{Display, Formatter},
    fs,
    ops::{Deref, DerefMut},
    str::FromStr,
};

use alloy_primitives::{Address, Bytes, B256, U64};
use ethers_core::{
    types::{BlockNumber, H256},
    utils::keccak256,
};
use serde::{Deserialize, Serialize};
use serde_json::Error;
use sqlx::prelude::FromRow;
use utoipa::ToSchema;

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq, FromRow, ToSchema)]
pub struct EthLog {
    pub removed: bool,
    #[serde(rename = "logIndex")]
    pub log_index: U64,
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

    pub events_signatures: Option<Vec<String>>, // aka filter0
    pub filter1: Option<Vec<String>>,
    pub filter2: Option<Vec<String>>,
    pub filter3: Option<Vec<String>>,
}

impl Criteria {
    pub fn from_file(file_path: &str) -> crate::Result<Criteria> {
        let contents = fs::read_to_string(file_path)
            .map_err(|e| crate::Error::InvalidCriteriasFile(e.to_string()))?;

        let criteria = serde_json::from_str(&contents)
            .map_err(|e| crate::Error::InvalidCriteriasFile(e.to_string()))?;

        Ok(criteria)
    }

    pub fn hashed_events(&self) -> Vec<H256> {
        self.events_signatures
            .clone()
            .unwrap_or_default()
            .into_iter()
            .map(|event| H256::from(keccak256(event)))
            .collect()
    }

    pub fn filter_as_h256(&self) -> Vec<H256> {
        self.filter1
            .clone()
            .unwrap_or_default()
            .into_iter()
            .map(|f| H256::from_str(&f).unwrap())
            .collect()
    }
}

impl Display for Criteria {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct Criterias(pub Vec<Criteria>);
impl Criterias {
    pub fn new(criterias: Option<Vec<Criteria>>) -> Self {
        match criterias {
            Some(criterias) => Self(criterias),
            None => Self::default(),
        }
    }

    pub fn criterias(&self) -> &Vec<Criteria> {
        &self.0
    }

    pub fn from_file(file_path: &str) -> crate::Result<Criterias> {
        let contents = fs::read_to_string(file_path)
            .map_err(|e| crate::Error::InvalidCriteriasFile(e.to_string()))?;

        let criterias = serde_json::from_str(&contents)
            .map_err(|e| crate::Error::InvalidCriteriasFile(e.to_string()))?;

        Ok(criterias)
    }
}

impl Display for Criterias {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self.0).unwrap())
    }
}

impl Iterator for Criterias {
    type Item = Criteria;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl Deref for Criterias {
    type Target = Vec<Criteria>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Criterias {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl FromStr for Criterias {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(serde_json::from_str(s)?))
    }
}

impl From<&str> for Criterias {
    fn from(s: &str) -> Self {
        Self(serde_json::from_str(s).unwrap())
    }
}

//impl From<&Criteria> for Filter {
//    fn from(criteria: &Criteria) -> Self {
//        Filter::new()
//            .address(ValueOrArray::Array(
//                criteria.addresses.clone().unwrap_or_default(),
//            ))
//            .topic0(ValueOrArray::Array(criteria.hashed_events()))
//            .topic1(ValueOrArray::Array(
//                criteria
//                    .clone()
//                    .filter1
//                    .unwrap_or_default()
//                    .into_iter()
//                    .map(|f| H256::from_str(&f).unwrap())
//                    .collect(),
//            ))
//            .topic2(ValueOrArray::Array(
//                criteria
//                    .clone()
//                    .filter2
//                    .unwrap_or_default()
//                    .into_iter()
//                    .map(|f| H256::from_str(&f).unwrap())
//                    .collect(),
//            ))
//            .topic3(ValueOrArray::Array(
//                criteria
//                    .clone()
//                    .filter3
//                    .unwrap_or_default()
//                    .into_iter()
//                    .map(|f| H256::from_str(&f).unwrap())
//                    .collect(),
//            ))
//            .from_block(criteria.src_block.unwrap_or(BlockNumber::Earliest))
//            .to_block(criteria.dst_block.unwrap_or(BlockNumber::Latest))
//    }
//}

#[cfg(test)]
mod tests {
    use ethers_core::types::{H160, U64};

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

        assert!(serde_json::from_value::<Log>(json).is_ok());
    }

    #[test]
    fn test_deserialize_empty_log() {
        let json = serde_json::json!({});

        assert!(serde_json::from_value::<Log>(json).is_err());
    }

    #[test]
    fn test_deserialize_criteria() {
        let json = serde_json::json!(
        {
            "name": "test",
            "srcBlock": "1",
            "dstBlock": "2",
            "addresses": ["0x0000000000000000000000000000000000000001", "0x0000000000000000000000000000000000000002"],
            "eventsSignatures": ["Transfer(address,address,uint256)"],
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
                H160::from_str("0x0000000000000000000000000000000000000001").unwrap(),
                H160::from_str("0x0000000000000000000000000000000000000002").unwrap(),
            ]),
        );
        assert_eq!(
            res.events_signatures,
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
