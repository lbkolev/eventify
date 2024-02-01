#![doc = include_str!("../README.md")]
#![warn(missing_debug_implementations, unreachable_pub, rustdoc::all)]
#![deny(unused_must_use, rust_2018_idioms)]

pub mod block;
pub mod consts;
pub mod contract;
pub mod error;
pub mod log;
pub mod transaction;

use std::collections::HashSet;

pub use block::EthBlock;
pub use contract::Contract;
pub use error::Error;
pub use log::{Criteria, EthLog};
use serde::Deserialize;
pub use transaction::{EthTransaction, TransactionResponse};

pub mod eth {
    pub use crate::{
        block::EthBlock as Block, log::EthLog as Log, transaction::EthTransaction as Transaction,
    };
}

type Result<T> = std::result::Result<T, error::Error>;

#[derive(Clone, Debug)]
pub struct NetworkKindError(String);

impl std::error::Error for NetworkKindError {}
impl std::fmt::Display for NetworkKindError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid network: {}", self.0)
    }
}

#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
pub enum NetworkKind {
    #[default]
    Ethereum,
}

impl std::fmt::Display for NetworkKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NetworkKind::Ethereum => write!(f, "eth"),
        }
    }
}

impl std::str::FromStr for NetworkKind {
    type Err = NetworkKindError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "ethereum" | "eth" => Ok(NetworkKind::Ethereum),
            _ => Err(NetworkKindError(s.to_string())),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Deserialize)]
pub enum ResourceKind {
    Block,
    Transaction,
    Log,
}

impl ResourceKind {
    pub fn resources_from_string(s: String) -> HashSet<ResourceKind> {
        s.split(',')
            .map(|x| match x.trim().to_lowercase().as_str() {
                "block" | "blocks" => ResourceKind::Block,
                "tx" | "txs" | "transactions" => ResourceKind::Transaction,
                "log" | "logs" => ResourceKind::Log,
                _ => {
                    panic!("invalid resource: {}", x);
                }
            })
            .collect()
    }

    pub fn resources_from_str(s: &str) -> HashSet<ResourceKind> {
        ResourceKind::resources_from_string(s.to_string())
    }
}
