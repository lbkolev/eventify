use std::error::Error;

use alloy_primitives::BlockNumber;

use crate::{
    types::{NodeClient, StorageClient},
    Collector,
};
use eventify_primitives::Criterias;

#[trait_variant::make(Run: Send)]
pub trait LocalRun {
    async fn run<N, S, E>(
        processor: Collector<N, S>,
        skip_transactions: bool,
        skip_blocks: bool,
        src_block: BlockNumber,
        dst_block: BlockNumber,
        criterias: Option<Criterias>,
    ) -> Result<(), E>
    where
        E: Error + Send + Sync,
        N: NodeClient,
        S: StorageClient;
}
