use std::{fs, str::FromStr};

use ethers_core::{
    types::{Address, Bytes, Filter, Log, ValueOrArray, H256, U256, U64},
    utils::keccak256,
};
use serde::{Deserialize, Serialize};
use serde_json::Error;
use sqlx::prelude::FromRow;

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct IndexedLog(Log);

impl From<Log> for IndexedLog {
    fn from(log: Log) -> Self {
        Self(log)
    }
}

impl IndexedLog {
    pub fn address(&self) -> Address {
        self.0.address
    }

    pub fn topics(&self) -> Vec<H256> {
        self.0.topics.clone()
    }

    pub fn data(&self) -> Bytes {
        self.0.data.clone()
    }

    pub fn block_hash(&self) -> Option<H256> {
        self.0.block_hash
    }

    pub fn block_number(&self) -> Option<U64> {
        self.0.block_number
    }

    pub fn transaction_hash(&self) -> Option<H256> {
        self.0.transaction_hash
    }

    pub fn transaction_index(&self) -> Option<U64> {
        self.0.transaction_index
    }

    pub fn transaction_log_index(&self) -> Option<U256> {
        self.0.transaction_log_index
    }

    pub fn log_index(&self) -> Option<U256> {
        self.0.log_index
    }

    pub fn log_type(&self) -> Option<&String> {
        self.0.log_type.as_ref()
    }

    pub fn removed(&self) -> Option<bool> {
        self.0.removed
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Criteria {
    pub name: String,
    pub events: Vec<String>,
    pub addresses: Vec<Address>,
}

impl Criteria {
    pub fn new(name: String, events: Vec<String>, addresses: Vec<Address>) -> Self {
        Self {
            name,
            events,
            addresses,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn events(&self) -> &Vec<String> {
        &self.events
    }

    pub fn addresses(&self) -> &Vec<Address> {
        &self.addresses
    }

    pub fn read_criteria_from_file(file_path: &str) -> crate::Result<Criteria> {
        let contents = fs::read_to_string(file_path)
            .map_err(|e| crate::Error::InvalidCriteriasFile(e.to_string()))?;

        let criteria = serde_json::from_str(&contents)
            .map_err(|e| crate::Error::InvalidCriteriasFile(e.to_string()))?;

        Ok(criteria)
    }

    pub fn hashed_events(&self) -> Vec<H256> {
        self.events
            .clone()
            .into_iter()
            .map(|event| H256::from(keccak256(event)))
            .collect()
    }
}

/// Set of events and addresses
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Criterias(pub Vec<Criteria>);

impl Criterias {
    pub fn new(criterias: Vec<Criteria>) -> Self {
        Self(criterias)
    }

    pub fn criterias(&self) -> &Vec<Criteria> {
        &self.0
    }

    pub fn read_criterias_from_file(file_path: &str) -> crate::Result<Criterias> {
        let contents = fs::read_to_string(file_path)
            .map_err(|e| crate::Error::InvalidCriteriasFile(e.to_string()))?;

        let criterias = serde_json::from_str(&contents)
            .map_err(|e| crate::Error::InvalidCriteriasFile(e.to_string()))?;

        Ok(criterias)
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

impl From<&Criteria> for Filter {
    fn from(criteria: &Criteria) -> Self {
        Filter::new()
            .topic0(ValueOrArray::Array(criteria.hashed_events()))
            .address(ValueOrArray::Array(criteria.addresses.clone()))
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

        serde_json::from_value::<IndexedLog>(json).unwrap();
    }

    //#[test]
    //fn test_read_criterias_from_file() {
    //    let criterias = Criterias::read_criterias_from_file("tests/criterias.json").unwrap();

    //    assert_eq!(criterias.criterias().len(), 2);
    //    assert_eq!(criterias.criterias()[0].name(), "test1");
    //    assert_eq!(criterias.criterias()[0].events().len(), 2);
    //    assert_eq!(criterias.criterias()[0].addresses().len(), 2);
    //    assert_eq!(criterias.criterias()[1].name(), "test2");
    //    assert_eq!(criterias.criterias()[1].events().len(), 1);
    //    assert_eq!(criterias.criterias()[1].addresses().len(), 1);
    //}

    //#[test]
    //fn test_read_criteria_from_file() {
    //    let criteria = Criteria::read_criteria_from_file("tests/criteria.json").unwrap();

    //    assert_eq!(criteria.name(), "test1");
    //    assert_eq!(criteria.events().len(), 2);
    //    assert_eq!(criteria.addresses().len(), 2);
    //}
}
