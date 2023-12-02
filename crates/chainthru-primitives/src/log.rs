use std::{fs, str::FromStr};

use ethers_core::{
    types::{Address, Bytes, Filter, Log, ValueOrArray, H256, U256, U64},
    utils::keccak256,
};
use serde::{Deserialize, Serialize};
use serde_json::Error;

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
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

/// Set of events and addresses
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
