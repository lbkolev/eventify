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

type Result<T> = std::result::Result<T, error::Error>;
