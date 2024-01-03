#![doc = include_str!("../README.md")]
#![warn(missing_debug_implementations, unreachable_pub, rustdoc::all)]
#![deny(unused_must_use, rust_2018_idioms)]

pub mod clients;
pub mod collector;
pub mod error;
pub mod macros;
pub mod manager;
pub mod types;

use std::fmt::Display;

pub use collector::Collector;
pub use error::{Error, NodeClientError};
pub use manager::Manager;
pub use types::run::Run;

type Result<T> = std::result::Result<T, error::Error>;

// Supported chains
#[derive(Clone, Copy, Debug, Default)]
pub enum ChainKind {
    #[default]
    Ethereum,
}

impl Display for ChainKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChainKind::Ethereum => write!(f, "eth"),
        }
    }
}

impl std::str::FromStr for ChainKind {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "ethereum" | "eth" => Ok(ChainKind::Ethereum),
            _ => Err(Error::InvalidChainKind(s.to_string())),
        }
    }
}

// Supported storages
#[derive(Clone, Copy, Debug, Default)]
pub enum StorageKind {
    #[default]
    Postgres,
}

impl Display for StorageKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StorageKind::Postgres => write!(f, "postgres"),
        }
    }
}

impl std::str::FromStr for StorageKind {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "postgres" | "pg" => Ok(StorageKind::Postgres),
            _ => Err(Error::InvalidDatabase(s.to_string())),
        }
    }
}
