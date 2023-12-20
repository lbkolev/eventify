#![doc = include_str!("../README.md")]
#![warn(missing_debug_implementations, unreachable_pub, rustdoc::all)]
#![deny(unused_must_use, rust_2018_idioms)]

pub mod block;
pub mod config;
pub mod contract;
pub mod error;
pub mod log;
pub mod storage;
pub mod transaction;

pub use block::Block;
pub use config::database::DatabaseConfig;
pub use contract::Contract;
pub use error::Error;
pub use log::{Criteria, Criterias, Log};
pub use storage::{Auth, Storage};
pub use transaction::Transaction;

/// The result type used through the application code.
type Result<T> = std::result::Result<T, error::Error>;

pub(crate) use ethers_core::types::{
    Block as ETHBlock, Log as ETHLog, Transaction as ETHTransaction,
};
