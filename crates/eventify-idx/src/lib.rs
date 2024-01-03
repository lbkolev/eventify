#![doc = include_str!("../README.md")]
#![warn(missing_debug_implementations, unreachable_pub, rustdoc::all)]
#![deny(unused_must_use, rust_2018_idioms)]

pub mod clients;
pub mod collector;
pub mod error;
pub mod macros;
pub mod manager;

pub use collector::{Collect, Collector};
pub use error::{Error, NodeClientError, StorageClientError};
pub use manager::{Manager, Run};

type Result<T> = std::result::Result<T, error::Error>;
