use std::error::Error;

use alloy_primitives::BlockNumber;

use crate::{
    types::{NodeClient, StorageClient},
    Collector,
};
use eventify_primitives::Criterias;

#[async_trait::async_trait]
pub trait Run {
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
