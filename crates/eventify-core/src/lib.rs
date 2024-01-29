#![doc = include_str!("../README.md")]
#![warn(missing_debug_implementations, unreachable_pub, rustdoc::all)]
#![deny(unused_must_use, rust_2018_idioms)]

use futures::Future;

pub mod collector;
pub mod emit;
pub mod error;
pub mod macros;
pub mod manager;
pub mod networks;
pub mod provider;
pub mod store;

pub use collector::Collector;
pub use error::Error;
pub use manager::Manager;
pub use networks::eth;
pub use provider::{Node, NodeError};
pub use store::{Store, StoreError};

type Result<T> = std::result::Result<T, error::Error>;

use alloy_primitives::{BlockNumber, B256};
use eventify_primitives::{Contract, Criteria, EthBlock, EthLog, EthTransaction};
use std::fmt::Debug;

pub trait Storage: 'static + Clone + Debug + Sync + Send {
    fn store_block(
        &self,
        block: &EthBlock<B256>,
    ) -> impl Future<Output = std::result::Result<(), Error>> + Send;
    fn store_transaction(
        &self,
        transaction: &EthTransaction,
    ) -> impl Future<Output = std::result::Result<(), Error>> + Send;
    fn store_log(
        &self,
        log: &EthLog,
    ) -> impl Future<Output = std::result::Result<(), Error>> + Send;
    fn store_contract(
        &self,
        contract: &Contract,
    ) -> impl Future<Output = std::result::Result<(), Error>> + Send;
}

pub trait Collect<E>
where
    E: std::error::Error + Send + Sync,
{
    fn process_block(&self, b: BlockNumber) -> impl Future<Output = std::result::Result<(), E>>;
    fn process_blocks(
        &self,
        from: BlockNumber,
        to: BlockNumber,
    ) -> impl Future<Output = std::result::Result<(), E>>;
    fn process_logs(&self, criteria: &Criteria)
        -> impl Future<Output = std::result::Result<(), E>>;
    fn process_transactions(
        &self,
        b: BlockNumber,
    ) -> impl Future<Output = std::result::Result<(), E>>;
    fn process_transactions_from_range(
        &self,
        from: BlockNumber,
        to: BlockNumber,
    ) -> impl Future<Output = std::result::Result<(), E>>;

    fn stream_blocks(&self) -> impl Future<Output = std::result::Result<(), E>>;
    fn stream_transactions(&self) -> impl Future<Output = std::result::Result<(), E>>;
    fn stream_logs(&self) -> impl Future<Output = std::result::Result<(), E>>;
}
