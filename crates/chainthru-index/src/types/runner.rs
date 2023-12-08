use async_trait::async_trait;
use ethers_providers::JsonRpcClient;

use crate::Processor;
use chainthru_primitives::{Auth, Storage};

/// A trait for executing blockchain-related operations.
///
/// The `Runner` trait abstracts the logic for running operations
/// in both single-threaded and multi-threaded environments. It's designed
/// for applications that interact with blockchain data, allowing for
/// flexible implementation of data fetching, processing, and storage.
///
/// This trait is particularly useful for managing the execution flow
/// of blockchain data processing tasks, whether running sequentially or
/// in parallel (when enabled through feature flags).
//#[async_trait::async_trait]
#[async_trait]
pub trait Runner {
    type Error;

    /// Executes defined operations in a single-threaded manner.
    ///
    /// This method is responsible for iterating over blockchain blocks,
    /// processing logs, and handling data storage. It should be implemented
    /// to perform these operations sequentially.
    async fn run<T: JsonRpcClient + Clone + Send + Sync, U: Storage + Auth + Clone + Send + Sync>(
        processor: Processor<T, U>,
    ) -> std::result::Result<(), Self::Error>;

    /// Executes defined operations in a multi-threaded environment.
    ///
    /// Available when the `multi-thread` feature is enabled. This method
    /// handles the concurrent processing of logs and blocks, leveraging
    /// asynchronous programming and task spawning for improved performance.
    ///
    /// This method is only available when compiled with the `multi-thread`
    /// feature flag enabled.
    #[cfg(feature = "multi-thread")]
    async fn run_par<
        T: JsonRpcClient + Clone + Send + Sync,
        U: Storage + Auth + Clone + Send + Sync,
    >(
        processor: Processor<T, U>,
    ) -> std::result::Result<(), Self::Error>;
}
