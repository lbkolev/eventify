#![doc = include_str!("../README.md")]
#![warn(missing_debug_implementations, unreachable_pub, rustdoc::all)]
#![deny(unused_must_use, rust_2018_idioms)]

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
pub use store::{Storage, StoreError};

type Result<T> = std::result::Result<T, error::Error>;

use std::fmt::Debug;

use alloy_primitives::{Address, BlockNumber, Bytes, FixedBytes, B256, U64};
use futures::Future;
use tokio::sync::watch::Receiver;

use eventify_primitives::{Contract, Criteria, EthBlock, EthLog, EthTransaction};

pub trait Store: 'static + Clone + Debug + Sync + Send {
    fn store_block(
        &self,
        block: &EthBlock<B256>,
    ) -> impl Future<Output = std::result::Result<(), Error>> + Send;
    fn store_transaction(
        &self,
        transaction: &EthTransaction,
    ) -> impl Future<Output = std::result::Result<(), Error>> + Send;
    fn store_contract(
        &self,
        contract: &Contract,
    ) -> impl Future<Output = std::result::Result<(), Error>> + Send;
    fn store_log(
        &self,
        log: &EthLog,
    ) -> impl Future<Output = std::result::Result<(), Error>> + Send;
    fn store_log_transfer(
        &self,
        tx_hash: &FixedBytes<32>,
        from: &FixedBytes<32>,
        to: &FixedBytes<32>,
        value: Bytes,
    ) -> impl Future<Output = std::result::Result<(), Error>> + Send;
    fn store_log_approval(
        &self,
        tx_hash: &FixedBytes<32>,
        owner: &FixedBytes<32>,
        spender: &FixedBytes<32>,
        value: Bytes,
    ) -> impl Future<Output = std::result::Result<(), Error>> + Send;
    fn store_log_approval_for_all(
        &self,
        tx_hash: &FixedBytes<32>,
        owner: &FixedBytes<32>,
        operator: &FixedBytes<32>,
        approved: bool,
    ) -> impl Future<Output = std::result::Result<(), Error>> + Send;
    fn store_log_sent(
        &self,
        tx_hash: &FixedBytes<32>,
        operator: &FixedBytes<32>,
        from: &FixedBytes<32>,
        to: &FixedBytes<32>,
        amount: Bytes,
        data: Bytes,
        operator_data: Bytes,
    ) -> impl Future<Output = std::result::Result<(), Error>> + Send;
    fn store_log_minted(
        &self,
        tx_hash: &FixedBytes<32>,
        operator: &FixedBytes<32>,
        to: &FixedBytes<32>,
        amount: Bytes,
        data: Bytes,
        operator_data: Bytes,
    ) -> impl Future<Output = std::result::Result<(), Error>> + Send;
    fn store_log_burned(
        &self,
        tx_hash: &FixedBytes<32>,
        operator: &FixedBytes<32>,
        from: &FixedBytes<32>,
        amount: Bytes,
        data: Bytes,
        operator_data: Bytes,
    ) -> impl Future<Output = std::result::Result<(), Error>> + Send;
    fn store_log_authorized_operator(
        &self,
        tx_hash: &FixedBytes<32>,
        operator: &FixedBytes<32>,
        holder: &FixedBytes<32>,
    ) -> impl Future<Output = std::result::Result<(), Error>> + Send;
    fn store_log_revoked_operator(
        &self,
        tx_hash: &FixedBytes<32>,
        operator: &FixedBytes<32>,
        holder: &FixedBytes<32>,
    ) -> impl Future<Output = std::result::Result<(), Error>> + Send;
    fn store_log_transfer_single(
        &self,
        tx_hash: &FixedBytes<32>,
        operator: &FixedBytes<32>,
        from: &FixedBytes<32>,
        to: &FixedBytes<32>,
        id: U64,
        value: Bytes,
    ) -> impl Future<Output = std::result::Result<(), Error>> + Send;
    fn store_log_transfer_batch(
        &self,
        tx_hash: &FixedBytes<32>,
        operator: &FixedBytes<32>,
        from: &FixedBytes<32>,
        to: &FixedBytes<32>,
        ids: Vec<U64>,
        values: Vec<Bytes>,
    ) -> impl Future<Output = std::result::Result<(), Error>> + Send;
    fn store_log_uri(
        &self,
        tx_hash: &FixedBytes<32>,
        value: String,
        id: U64,
    ) -> impl Future<Output = std::result::Result<(), Error>> + Send;
    fn store_log_deposit(
        &self,
        tx_hash: &FixedBytes<32>,
        sender: &FixedBytes<32>,
        owner: &FixedBytes<32>,
        assets: U64,
        shares: U64,
    ) -> impl Future<Output = std::result::Result<(), Error>> + Send;
    fn store_log_withdraw(
        &self,
        tx_hash: &FixedBytes<32>,
        sender: &FixedBytes<32>,
        receiver: &FixedBytes<32>,
        owner: &FixedBytes<32>,
        assets: U64,
        shares: U64,
    ) -> impl Future<Output = std::result::Result<(), Error>> + Send;
}

pub trait Collect<E>
where
    E: std::error::Error + Send + Sync,
{
    fn collect_block(&self, b: BlockNumber) -> impl Future<Output = std::result::Result<(), E>>;
    fn collect_blocks(
        &self,
        signal_receiver: Receiver<bool>,
        from: BlockNumber,
        to: BlockNumber,
    ) -> impl Future<Output = std::result::Result<(), E>>;
    fn collect_logs(
        &self,
        signal_receiver: Receiver<bool>,
        criteria: &Criteria,
    ) -> impl Future<Output = std::result::Result<(), E>>;
    fn collect_transactions(
        &self,
        b: BlockNumber,
    ) -> impl Future<Output = std::result::Result<(), E>>;
    fn collect_transactions_from_range(
        &self,
        signal_receiver: Receiver<bool>,
        from: BlockNumber,
        to: BlockNumber,
    ) -> impl Future<Output = std::result::Result<(), E>>;

    fn stream_blocks(
        &self,
        signal_receiver: Receiver<bool>,
    ) -> impl Future<Output = std::result::Result<(), E>>;
    fn stream_transactions(
        &self,
        signal_receiver: Receiver<bool>,
    ) -> impl Future<Output = std::result::Result<(), E>>;
    fn stream_logs(
        &self,
        signal_receiver: Receiver<bool>,
    ) -> impl Future<Output = std::result::Result<(), E>>;
}
