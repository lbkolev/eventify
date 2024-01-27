#![doc = include_str!("../README.md")]
#![warn(missing_debug_implementations, unreachable_pub, rustdoc::all)]
#![deny(unused_must_use, rust_2018_idioms)]

pub mod collector;
pub mod emit;
pub mod error;
pub mod macros;
pub mod manager;
pub mod provider;
pub mod store;

pub use collector::Collector;
pub use error::Error;
pub use manager::Manager;
pub use provider::{eth::Eth, NodeProvider, NodeProviderError};
pub use store::{Store, StoreError};

type Result<T> = std::result::Result<T, error::Error>;

use alloy_primitives::{BlockNumber, B256};
use eventify_primitives::{Contract, Criteria, EthBlock, EthLog, EthTransaction};
use std::fmt::Debug;

#[trait_variant::make(StorageClient: Send)]
pub trait LocalStorageClient: 'static + Clone + Debug + Sync {
    async fn store_block(&self, block: &EthBlock<B256>) -> std::result::Result<(), Error>;
    async fn store_transaction(
        &self,
        transaction: &EthTransaction,
    ) -> std::result::Result<(), Error>;
    async fn store_log(&self, log: &EthLog) -> std::result::Result<(), Error>;
    async fn store_contract(&self, contract: &Contract) -> std::result::Result<(), Error>;
}

#[trait_variant::make(Collect: Send)]
pub trait LocalCollect<E>
where
    E: std::error::Error + Send + Sync,
{
    async fn process_block(&self, b: BlockNumber) -> std::result::Result<(), E>;
    async fn process_blocks(
        &self,
        from: BlockNumber,
        to: BlockNumber,
    ) -> std::result::Result<(), E>;
    async fn process_logs(&self, criteria: &Criteria) -> std::result::Result<(), E>;
    async fn process_transactions(&self, b: BlockNumber) -> std::result::Result<(), E>;
    async fn process_transactions_from_range(
        &self,
        from: BlockNumber,
        to: BlockNumber,
    ) -> std::result::Result<(), E>;

    async fn stream_blocks(&self) -> std::result::Result<(), E>;
    async fn stream_transactions(&self) -> std::result::Result<(), E>;
    async fn stream_logs(&self) -> std::result::Result<(), E>;
}
