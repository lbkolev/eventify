#![doc = include_str!("../README.md")]
#![warn(missing_debug_implementations, unreachable_pub, rustdoc::all)]
#![deny(unused_must_use, rust_2018_idioms)]

pub mod consts;
pub mod networks;
pub mod platform;

pub mod eth {
    pub use crate::networks::eth::{
        block::EthBlock as Block, log::EthLog as Log, transaction::EthTransaction as Transaction,
    };
}

pub mod traits {
    pub trait Block:
        Clone
        + std::fmt::Debug
        + Default
        + PartialEq
        + Eq
        + serde::Serialize
        + serde::de::DeserializeOwned
        + Send
        + Sync
    {
        fn parent_hash(&self) -> alloy_primitives::B256;
        fn hash(&self) -> Option<alloy_primitives::B256>;
        fn number(&self) -> Option<alloy_primitives::U64>;
    }

    pub trait Transaction:
        Clone
        + std::fmt::Debug
        + Default
        + PartialEq
        + Eq
        + serde::Serialize
        + serde::de::DeserializeOwned
        + Send
        + Sync
    {
        fn block_hash(&self) -> Option<alloy_primitives::B256>;
        fn hash(&self) -> alloy_primitives::B256;
    }

    pub trait Log:
        Clone
        + std::fmt::Debug
        + Default
        + PartialEq
        + Eq
        + serde::Serialize
        + serde::de::DeserializeOwned
        + Send
        + Sync
    {
        fn block_hash(&self) -> Option<alloy_primitives::B256>;
        fn block_number(&self) -> Option<alloy_primitives::U64>;

        fn transaction_hash(&self) -> Option<alloy_primitives::B256>;
        fn transaction_index(&self) -> Option<alloy_primitives::U64>;

        fn topics(&self) -> &Vec<alloy_primitives::B256>;
        fn data(&self) -> &alloy_primitives::Bytes;
        fn address(&self) -> &alloy_primitives::Address;
    }
}
