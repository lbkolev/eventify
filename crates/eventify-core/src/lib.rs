#![doc = include_str!("../README.md")]
#![warn(missing_debug_implementations, unreachable_pub, rustdoc::all)]
#![deny(unused_must_use, rust_2018_idioms)]

pub mod collector;
pub mod error;
pub mod manager;
pub mod networks;
pub mod queues;
pub mod storage;

pub use collector::Collector;
pub use error::Error;
pub use manager::Manager;
pub use networks::{eth, NetworkError};
pub use storage::{Storage, StorageError};

type Result<T> = std::result::Result<T, error::Error>;

pub mod traits {
    use std::{fmt::Debug, future::Future};
    use tokio::sync::watch::Receiver;

    use alloy_primitives::{BlockNumber, Bytes, FixedBytes, U64};
    use eyre::Result;
    use reconnecting_jsonrpsee_ws_client::Subscription;
    use serde::Serialize;

    use crate::{networks::NetworkError, Error};
    use eventify_primitives::{
        networks::{
            eth::{Contract, Criteria},
            NetworkKind, ResourceKind,
        },
        traits::{Block, Log, Transaction},
    };

    pub trait Collect<E>
    where
        E: std::error::Error + Send + Sync,
    {
        fn collect_block(&self, b: BlockNumber) -> impl Future<Output = Result<(), E>>;
        fn collect_blocks(
            &self,
            signal_receiver: Receiver<bool>,
            from: BlockNumber,
            to: BlockNumber,
        ) -> impl Future<Output = Result<(), E>>;
        fn collect_logs(
            &self,
            signal_receiver: Receiver<bool>,
            criteria: &Criteria,
        ) -> impl Future<Output = Result<(), E>>;
        fn collect_transactions(&self, b: BlockNumber) -> impl Future<Output = Result<(), E>>;
        fn collect_transactions_from_range(
            &self,
            signal_receiver: Receiver<bool>,
            from: BlockNumber,
            to: BlockNumber,
        ) -> impl Future<Output = Result<(), E>>;
        fn stream_blocks(
            &self,
            signal_receiver: Receiver<bool>,
        ) -> impl Future<Output = Result<(), E>>;
        fn stream_logs(
            &self,
            signal_receiver: Receiver<bool>,
        ) -> impl Future<Output = Result<(), E>>;
    }

    pub trait Network: 'static + Clone + Debug + Send + Sync {
        type Block: Block;
        type LightBlock: Block;
        type Transaction: Transaction;
        type Log: Log;

        fn get_block_number(
            &self,
        ) -> impl Future<Output = Result<BlockNumber, NetworkError>> + Send;
        // block with tx hashes
        fn get_block(
            &self,
            block: BlockNumber,
        ) -> impl Future<Output = Result<Self::LightBlock, NetworkError>> + Send;
        fn get_transactions(
            &self,
            n: BlockNumber,
        ) -> impl Future<Output = Result<Vec<Self::Transaction>, NetworkError>> + Send;
        fn get_logs(
            &self,
            criteria: &Criteria,
        ) -> impl Future<Output = Result<Vec<Self::Log>, NetworkError>> + Send;

        fn sub_blocks(&self) -> impl Future<Output = Result<Subscription, NetworkError>> + Send;
        fn sub_logs(&self) -> impl Future<Output = Result<Subscription, NetworkError>> + Send;
    }

    pub trait Store<N: Network>: 'static + Clone + Debug + Send + Sync {
        fn schema_name(&self) -> &str;

        fn store_block(
            &self,
            block: &N::LightBlock,
        ) -> impl Future<Output = Result<(), Error>> + Send;
        fn store_transaction(
            &self,
            transaction: &N::Transaction,
        ) -> impl Future<Output = Result<(), Error>> + Send;
        fn store_log(&self, log: &N::Log) -> impl Future<Output = Result<(), Error>> + Send;

        fn store_contract(
            &self,
            contract: &Contract,
        ) -> impl Future<Output = Result<(), Error>> + Send;
        fn store_log_transfer(
            &self,
            tx_hash: &FixedBytes<32>,
            from: &FixedBytes<32>,
            to: &FixedBytes<32>,
            value: Bytes,
        ) -> impl Future<Output = Result<(), Error>> + Send;
        fn store_log_approval(
            &self,
            tx_hash: &FixedBytes<32>,
            owner: &FixedBytes<32>,
            spender: &FixedBytes<32>,
            value: Bytes,
        ) -> impl Future<Output = Result<(), Error>> + Send;
        fn store_log_approval_for_all(
            &self,
            tx_hash: &FixedBytes<32>,
            owner: &FixedBytes<32>,
            operator: &FixedBytes<32>,
            approved: bool,
        ) -> impl Future<Output = Result<(), Error>> + Send;
        #[allow(clippy::too_many_arguments)]
        fn store_log_sent(
            &self,
            tx_hash: &FixedBytes<32>,
            operator: &FixedBytes<32>,
            from: &FixedBytes<32>,
            to: &FixedBytes<32>,
            amount: Bytes,
            data: Bytes,
            operator_data: Bytes,
        ) -> impl Future<Output = Result<(), Error>> + Send;
        fn store_log_minted(
            &self,
            tx_hash: &FixedBytes<32>,
            operator: &FixedBytes<32>,
            to: &FixedBytes<32>,
            amount: Bytes,
            data: Bytes,
            operator_data: Bytes,
        ) -> impl Future<Output = Result<(), Error>> + Send;
        fn store_log_burned(
            &self,
            tx_hash: &FixedBytes<32>,
            operator: &FixedBytes<32>,
            from: &FixedBytes<32>,
            amount: Bytes,
            data: Bytes,
            operator_data: Bytes,
        ) -> impl Future<Output = Result<(), Error>> + Send;
        fn store_log_authorized_operator(
            &self,
            tx_hash: &FixedBytes<32>,
            operator: &FixedBytes<32>,
            holder: &FixedBytes<32>,
        ) -> impl Future<Output = Result<(), Error>> + Send;
        fn store_log_revoked_operator(
            &self,
            tx_hash: &FixedBytes<32>,
            operator: &FixedBytes<32>,
            holder: &FixedBytes<32>,
        ) -> impl Future<Output = Result<(), Error>> + Send;
        fn store_log_transfer_single(
            &self,
            tx_hash: &FixedBytes<32>,
            operator: &FixedBytes<32>,
            from: &FixedBytes<32>,
            to: &FixedBytes<32>,
            id: U64,
            value: Bytes,
        ) -> impl Future<Output = Result<(), Error>> + Send;
        fn store_log_transfer_batch(
            &self,
            tx_hash: &FixedBytes<32>,
            operator: &FixedBytes<32>,
            from: &FixedBytes<32>,
            to: &FixedBytes<32>,
            ids: Vec<U64>,
            values: Vec<Bytes>,
        ) -> impl Future<Output = Result<(), Error>> + Send;
        fn store_log_uri(
            &self,
            tx_hash: &FixedBytes<32>,
            value: String,
            id: U64,
        ) -> impl Future<Output = Result<(), Error>> + Send;
        fn store_log_deposit(
            &self,
            tx_hash: &FixedBytes<32>,
            sender: &FixedBytes<32>,
            owner: &FixedBytes<32>,
            assets: U64,
            shares: U64,
        ) -> impl Future<Output = Result<(), Error>> + Send;
        fn store_log_withdraw(
            &self,
            tx_hash: &FixedBytes<32>,
            sender: &FixedBytes<32>,
            receiver: &FixedBytes<32>,
            owner: &FixedBytes<32>,
            assets: U64,
            shares: U64,
        ) -> impl Future<Output = Result<(), Error>> + Send;
    }

    pub trait Emit<N: Network>: 'static + Clone + Debug + Send + Sync {
        fn publish<T: Serialize>(
            &self,
            network: &NetworkKind,
            resource: &ResourceKind,
            message: &T,
        ) -> Result<()>;
    }
}
