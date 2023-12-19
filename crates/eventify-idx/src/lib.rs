#![doc = include_str!("../README.md")]

pub mod collector;
pub mod error;
pub mod macros;
pub mod manager;
pub mod providers;
pub mod types;

pub use collector::Collector;
pub use error::Error;
pub use manager::Manager;
pub use types::run::Run;

/// The Result used throughout the indexer
type Result<T> = std::result::Result<T, error::Error>;

// Supported chains
#[derive(Debug)]
pub enum Chain {
    Ethereum,
}
