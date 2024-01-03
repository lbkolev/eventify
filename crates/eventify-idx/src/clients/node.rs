pub mod eth;

use std::fmt::Display;

use alloy_primitives::BlockNumber;

use crate::{Error, NodeClientError};
use eventify_primitives::{Block, Criteria, Log, Transaction};

#[async_trait::async_trait]
pub trait NodeClient: 'static + Clone + Send + Sync {
    async fn get_block_number(&self) -> Result<u64, NodeClientError>;
    async fn get_block(&self, block: BlockNumber) -> Result<Block, NodeClientError>;
    async fn get_transactions(
        &self,
        block: BlockNumber,
    ) -> Result<Vec<Transaction>, NodeClientError>;
    async fn get_logs(&self, criterias: &Criteria) -> Result<Vec<Log>, NodeClientError>;
}

#[async_trait::async_trait]
pub trait Auth {
    async fn connect(url: &str) -> Self;
}

#[cfg(feature = "eth")]
pub mod eth_client {
    use super::Auth;
    use crate::{impl_eth, node_client};
    use ethers_providers::{Middleware, Provider};

    #[cfg(feature = "http")]
    pub mod http {
        use super::*;
        use crate::clients::NodeAuth as Auth;
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

pub use eth_client::{http::EthHttp, ipc::EthIpc, ws::EthWs};

// Supported chains
#[derive(Clone, Copy, Debug, Default)]
pub enum NodeKind {
    #[default]
    Ethereum,
}

impl Display for NodeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeKind::Ethereum => write!(f, "eth"),
        }
    }
}

impl std::str::FromStr for NodeKind {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "ethereum" | "eth" => Ok(NodeKind::Ethereum),
            _ => Err(Error::InvalidNodeKind(s.to_string())),
        }
    }
}

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
