use std::{fs, str::FromStr};

use ethers_core::{
    types::{Address, Filter, ValueOrArray, H256},
    utils::keccak256,
};
use serde::{Deserialize, Serialize};
use serde_json::Error;

/// Set of events and addresses
#[derive(Debug, Serialize, Deserialize, Clone)]
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

    pub fn read_criterias_from_file(file_path: &str) -> crate::Result<Vec<Criteria>> {
        let contents = fs::read_to_string(file_path)
            .map_err(|e| crate::Error::InvalidCriteriasFile(e.to_string()))?;

        let criterias = serde_json::from_str(&contents)
            .map_err(|e| crate::Error::InvalidCriteriasFile(e.to_string()))?;

        Ok(criterias)
    }

    pub fn hashed_events(&self) -> Vec<H256> {
        self.events
            .clone()
            .into_iter()
            .map(|event| H256::from(keccak256(event)))
            .collect()
    }
}

impl FromStr for Criteria {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

impl From<&Criteria> for Filter {
    fn from(criteria: &Criteria) -> Self {
        Filter::new()
            .topic0(ValueOrArray::Array(criteria.hashed_events()))
            .address(ValueOrArray::Array(criteria.addresses.clone()))
    }
}
