#![doc = include_str!("../README.md")]
#![warn(missing_debug_implementations, unreachable_pub, rustdoc::all)]
#![deny(unused_must_use, rust_2018_idioms)]

pub mod consts;
pub mod error;
pub mod network;
pub mod platform;

pub use error::Error;

pub mod eth {
    pub use crate::network::{
        block::EthBlock as Block, log::EthLog as Log, transaction::EthTransaction as Transaction,
    };
}

type Result<T> = std::result::Result<T, error::Error>;
