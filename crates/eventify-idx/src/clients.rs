pub mod node;
pub mod storage;
pub use node::{Auth as NodeAuth, NodeClient};
pub use storage::{Auth as StorageAuth, StorageClient};

use alloy_primitives::BlockNumber;

use crate::NodeClientError;
use eventify_primitives::{Block, Criteria, Log, Transaction};

#[cfg(feature = "eth")]
pub mod eth {
    use crate::{clients::NodeAuth as Auth, impl_eth, node_client};
    use ethers_providers::{Middleware, Provider};

    #[cfg(feature = "http")]
    pub mod http {
        use super::*;
        use ethers_providers::Http;

        node_client!(EthHttp, Provider<Http>);
        impl_eth!(EthHttp);
    }

    #[cfg(feature = "ws")]
    pub mod ws {
        use super::*;
        use ethers_providers::Ws;

        node_client!(EthWs, Provider<Ws>);
        impl_eth!(EthWs);
    }

    #[cfg(feature = "ipc")]
    pub mod ipc {
        use super::*;
        use ethers_providers::Ipc;

        node_client!(EthIpc, Provider<Ipc>);
        impl_eth!(EthIpc);
    }
}

pub use eth::{http::EthHttp, ipc::EthIpc, ws::EthWs};

#[cfg(all(feature = "eth", feature = "ipc"))]
#[derive(Debug, Clone)]
pub enum NodeClientKind {
    EthHttp(EthHttp),
    EthWs(EthWs),
    EthIpc(EthIpc),
}

#[async_trait::async_trait]
impl NodeClient for NodeClientKind {
    async fn get_block_number(&self) -> Result<u64, NodeClientError> {
        match self {
            NodeClientKind::EthHttp(inner) => inner.get_block_number().await,
            NodeClientKind::EthWs(inner) => inner.get_block_number().await,
            NodeClientKind::EthIpc(inner) => inner.get_block_number().await,
        }
    }

    async fn get_block(&self, block: BlockNumber) -> Result<Block, NodeClientError> {
        match self {
            NodeClientKind::EthHttp(inner) => inner.get_block(block).await,
            NodeClientKind::EthWs(inner) => inner.get_block(block).await,
            NodeClientKind::EthIpc(inner) => inner.get_block(block).await,
        }
    }

    async fn get_transactions(
        &self,
        block: BlockNumber,
    ) -> Result<Vec<Transaction>, NodeClientError> {
        match self {
            NodeClientKind::EthHttp(inner) => inner.get_transactions(block).await,
            NodeClientKind::EthWs(inner) => inner.get_transactions(block).await,
            NodeClientKind::EthIpc(inner) => inner.get_transactions(block).await,
        }
    }

    async fn get_logs(&self, criterias: &Criteria) -> Result<Vec<Log>, NodeClientError> {
        match self {
            NodeClientKind::EthHttp(inner) => inner.get_logs(criterias).await,
            NodeClientKind::EthWs(inner) => inner.get_logs(criterias).await,
            NodeClientKind::EthIpc(inner) => inner.get_logs(criterias).await,
        }
    }
}
