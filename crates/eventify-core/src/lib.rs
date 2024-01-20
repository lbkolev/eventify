#![doc = include_str!("../README.md")]
#![warn(missing_debug_implementations, unreachable_pub, rustdoc::all)]
#![deny(unused_must_use, rust_2018_idioms)]

pub mod collector;
pub mod error;
pub mod macros;
pub mod manager;
pub mod provider;
pub mod storage;

pub use collector::Collector;
pub use error::{Error, StorageClientError};
pub use manager::{Manager, Run};
pub use provider::{eth::Eth, NodeProvider, NodeProviderError};

type Result<T> = std::result::Result<T, error::Error>;

use alloy_primitives::BlockNumber;
use eventify_primitives::{Contract, Criteria, EthBlock, EthLog, EthTransaction};
use std::fmt::Debug;

#[trait_variant::make(StorageClient: Send)]
pub trait LocalStorageClient: 'static + Clone + Debug + Sync {
    async fn store_block(&self, block: &EthBlock) -> std::result::Result<(), Error>;
    async fn store_transaction(
        &self,
        transaction: &EthTransaction,
    ) -> std::result::Result<(), Error>;
    async fn store_log(&self, log: &EthLog) -> std::result::Result<(), Error>;
    async fn store_contract(&self, contract: &Contract) -> std::result::Result<(), Error>;
}

#[trait_variant::make(Auth: Send)]
pub trait LocalAuth {
    async fn connect(url: &str) -> Self;
}

/// `Collect` Trait
///
/// An asynchronous trait designed for processing various types of data.
/// Implementers of this trait typically handle tasks such as fetching,
/// parsing, and storing data asynchronously. The trait provides a flexible
/// interface for different kinds of data processing activities, allowing
/// implementers to define the specifics of these activities.
#[trait_variant::make(Collect: Send)]
pub trait LocalCollect<T, E>
where
    T: Into<Criteria>,
    E: std::error::Error + Send + Sync,
{
    async fn process_block(&self, b: BlockNumber) -> std::result::Result<(), E>;
    async fn process_blocks(
        &self,
        from: BlockNumber,
        to: BlockNumber,
    ) -> std::result::Result<(), E>;
    async fn process_logs(&self, c: T) -> std::result::Result<(), E>;
    async fn process_transactions(&self, b: BlockNumber) -> std::result::Result<(), E>;
    async fn process_transactions_from_range(
        &self,
        from: BlockNumber,
        to: BlockNumber,
    ) -> std::result::Result<(), E>;
}
