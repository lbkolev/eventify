#![doc = include_str!("../README.md")]
#![warn(missing_debug_implementations, unreachable_pub, rustdoc::all)]
#![deny(unused_must_use, rust_2018_idioms)]

pub mod criteria;
pub mod events;
pub mod networks;

pub mod ethereum {
    pub use crate::networks::ethereum::{block::EthBlock as Block, log::EthLog as Log};
}

pub mod zksync {
    pub use crate::networks::zksync::{block::ZksyncBlock as Block, log::ZksyncLog as Log};
}

pub mod polygon {
    pub use crate::networks::polygon::{block::PolygonBlock as Block, log::PolygonLog as Log};
}

pub mod optimism {
    pub use crate::networks::optimism::{block::OptimismBlock as Block, log::OptimismLog as Log};
}

pub mod arbitrum {
    pub use crate::networks::arbitrum::{block::ArbitrumBlock as Block, log::ArbitrumLog as Log};
}

pub mod linea {
    pub use crate::networks::linea::{block::LineaBlock as Block, log::LineaLog as Log};
}

pub mod avalanche {
    pub use crate::networks::avalanche::{
        block::AvalancheBlock as Block, log::AvalancheLog as Log,
    };
}

pub mod bsc {
    pub use crate::networks::bsc::{block::BscBlock as Block, log::BscLog as Log};
}

pub mod base {
    pub use crate::networks::base::{block::BaseBlock as Block, log::BaseLog as Log};
}

pub use traits::{Block as BlockT, Emit as EmitT, Insert as InsertT, Log as LogT};

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
        fn core(&self) -> &crate::networks::core::CoreBlock;
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
        fn core(&self) -> &crate::networks::core::CoreLog;
    }
}
