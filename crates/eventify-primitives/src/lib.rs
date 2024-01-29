#![doc = include_str!("../README.md")]
#![warn(missing_debug_implementations, unreachable_pub, rustdoc::all)]
#![deny(unused_must_use, rust_2018_idioms)]

pub mod block;
pub mod consts;
pub mod contract;
pub mod error;
pub mod log;
pub mod transaction;

pub use block::EthBlock;
pub use contract::Contract;
pub use error::Error;
pub use log::{Criteria, EthLog};
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

impl NetworkKind {
    pub fn supported_resources(&self) -> Vec<ResourceKind> {
        let mut allowed = vec![];

        match self {
            &NetworkKind::Ethereum => {
                allowed.push(ResourceKind::Block);
                allowed.push(ResourceKind::Transaction);
                allowed.push(ResourceKind::Log);
            }
        }

        allowed
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub enum ResourceKind {
    Block,
    Transaction,
    Log,
    ERC_APPROVAL,
    ERC_TRANSFER,
    ERC_APPROVAL_FOR_ALL,
}

impl std::fmt::Display for ResourceKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResourceKind::Block => write!(f, "block"),
            ResourceKind::Transaction => write!(f, "transaction"),
            ResourceKind::Log => write!(f, "log"),
            ResourceKind::ERC_APPROVAL => write!(f, "erc-approval"),
            ResourceKind::ERC_TRANSFER => write!(f, "erc-transfer"),
            ResourceKind::ERC_APPROVAL_FOR_ALL => write!(f, "erc-approval-for-all"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Bundle {
    pub network: NetworkKind,
    pub resource: ResourceKind,
}

impl Bundle {
    pub fn new(network: NetworkKind, resource: ResourceKind) -> Self {
        Self { network, resource }
    }
}

impl std::fmt::Display for Bundle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.network, self.resource)
    }
}
