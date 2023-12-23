#![doc = include_str!("../README.md")]
#![warn(missing_debug_implementations, unreachable_pub, rustdoc::all)]
#![deny(unused_must_use, rust_2018_idioms)]

pub mod collector;
pub mod error;
pub mod macros;
pub mod manager;
pub mod providers;
pub mod types;

use std::fmt::Display;

pub use collector::Collector;
pub use error::Error;
pub use manager::Manager;
pub use types::run::Run;

/// The Result used throughout the indexer
type Result<T> = std::result::Result<T, error::Error>;

// Supported chains
#[derive(Clone, Copy, Debug, Default)]
pub enum Chain {
    #[default]
    Ethereum,
}

impl Display for Chain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Chain::Ethereum => write!(f, "eth"),
        }
    }
}

impl std::str::FromStr for Chain {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "ethereum" | "eth" => Ok(Chain::Ethereum),
            _ => Err(Error::InvalidChain(s.to_string())),
        }
    }
}

// Supported storages
#[derive(Clone, Copy, Debug, Default)]
pub enum Database {
    #[default]
    Postgres,
}

impl Display for Database {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Database::Postgres => write!(f, "postgres"),
        }
    }
}

impl std::str::FromStr for Database {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "postgres" | "pg" => Ok(Database::Postgres),
            _ => Err(Error::InvalidDatabase(s.to_string())),
        }
    }
}
