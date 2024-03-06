#![doc = include_str!("../README.md")]
#![warn(missing_debug_implementations, unreachable_pub, rustdoc::all)]
#![deny(unused_must_use, rust_2018_idioms)]

pub mod events;
pub mod networks;

pub mod eth {
    pub use crate::networks::ethereum::{
        block::EthBlock as Block, log::EthLog as Log, transaction::EthTransaction as Transaction,
    };
}

pub use traits::{
    Block as BlockT, Emit as EmitT, Insert as InsertT, Log as LogT, Transaction as TransactionT,
};

#[derive(thiserror::Error, Debug)]
pub enum EmitError {
    #[error(transparent)]
    RedisError(#[from] redis::RedisError),
    #[error(transparent)]
    SerdeError(#[from] serde_json::Error),
}

mod traits {
    pub trait Insert: Sync + Send {
        fn insert(
            &self,
            pool: &sqlx::PgPool,
            schema: &str,
            tx_hash: &Option<alloy_primitives::B256>,
        ) -> impl std::future::Future<Output = eyre::Result<(), sqlx::Error>> + Send;
    }

    pub trait Emit: Sync + Send {
        fn emit(
            &self,
            queue: &redis::Client,
            network: &crate::networks::NetworkKind,
        ) -> impl std::future::Future<Output = eyre::Result<(), super::EmitError>> + Send;
    }

    pub trait Block:
        Insert
        + Emit
        + Clone
        + std::fmt::Debug
        + Default
        + PartialEq
        + Eq
        + std::hash::Hash
        + serde::Serialize
        + serde::de::DeserializeOwned
        + Sync
        + Send
    {
        fn hash(&self) -> Option<alloy_primitives::B256>;
        fn number(&self) -> Option<alloy_primitives::U64>;
        fn parent_hash(&self) -> alloy_primitives::B256;
    }

    pub trait Transaction:
        Insert
        + Emit
        + Clone
        + std::fmt::Debug
        + Default
        + PartialEq
        + Eq
        + std::hash::Hash
        + serde::Serialize
        + serde::de::DeserializeOwned
        + Sync
        + Send
    {
        fn hash(&self) -> alloy_primitives::B256;
        fn block_hash(&self) -> Option<alloy_primitives::B256>;
    }

    pub trait Log:
        Insert
        + Emit
        + Clone
        + std::fmt::Debug
        + Default
        + PartialEq
        + Eq
        + std::hash::Hash
        + serde::Serialize
        + serde::de::DeserializeOwned
        + Sync
        + Send
    {
        fn block_hash(&self) -> Option<alloy_primitives::B256>;
        fn block_number(&self) -> Option<alloy_primitives::U64>;

        fn tx_hash(&self) -> Option<alloy_primitives::B256>;
        fn tx_index(&self) -> Option<alloy_primitives::U64>;

        fn data(&self) -> &alloy_primitives::Bytes;
        fn topics(&self) -> &Vec<alloy_primitives::B256>;
        fn address(&self) -> &alloy_primitives::Address;
    }
}
