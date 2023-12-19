use std::error::Error;

use alloy_primitives::BlockNumber;
use ethers_providers::JsonRpcClient;

use crate::Collector;
use eventify_primitives::{Criterias, Storage};

use super::provider::NodeProvider;

/// A trait for executing blockchain-related operations.
///
/// The `Run` trait abstracts the logic for running operations
/// in both single-threaded and multi-threaded environments. It's designed
/// for applications that interact with blockchain data, allowing for
/// flexible implementation of data fetching, processing, and storage.
///
/// This trait is particularly useful for managing the execution flow
/// of blockchain data processing tasks, whether running sequentially or
/// in parallel (when enabled through feature flags).
#[async_trait::async_trait]
pub trait Run {
    /// Executes defined operations in a single-threaded manner.
    ///
    /// This method is responsible for iterating over blockchain blocks,
    /// processing logs, and handling data storage. It should be implemented
    /// to perform these operations sequentially.
    async fn run<N, S, E>(
        processor: Collector<N, S>,
        src_block: BlockNumber,
        dst_block: BlockNumber,
        criterias: Option<Criterias>,
    ) -> Result<(), E>
    where
        E: Error + Send + Sync,
        N: NodeProvider<crate::Error>,
        S: Storage;

    /// Executes defined operations in a multi-threaded environment.
    ///
    /// Available when the `multi-thread` feature is enabled. This method
    /// handles the concurrent processing of logs and blocks, leveraging
    /// asynchronous programming and task spawning for improved performance.
    ///
    /// This method is only available when compiled with the `multi-thread`
    /// feature flag enabled.
    #[cfg(feature = "multi-thread")]
    async fn run_par<N, S, E>(
        processor: Collector<N, S>,
        src_block: BlockNumber,
        dst_block: BlockNumber,
        criterias: Option<Criterias>,
    ) -> Result<(), E>
    where
        E: Error + Send + Sync,
        N: NodeProvider<crate::Error>,
        S: Storage;
}
