#![doc = include_str!("../README.md")]

pub mod block;
pub mod contract;
pub mod database;
pub mod error;
pub mod log;
pub mod macros;
pub mod storage;
pub mod transaction;

pub use block::IndexedBlock;
pub use contract::Contract;
pub use database::Settings as DatabaseSettings;
pub use error::Error;
pub use log::{Criteria, Criterias};
pub use storage::{Auth, Storage};
pub use transaction::IndexedTransaction;

/// The result type used through the application code.
type Result<T> = std::result::Result<T, error::Error>;
