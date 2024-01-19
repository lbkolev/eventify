pub mod eth;

use std::fmt::Display;

use crate::{Error, NodeClientError};
use alloy_primitives::BlockNumber;
use eventify_primitives::{Criteria, EthBlock, EthLog, EthTransaction};

#[async_trait::async_trait]
pub trait Auth {
    async fn connect(url: &str) -> Self;
}

#[async_trait::async_trait]
pub trait NodeClient: 'static + Clone + Send + Sync {
    async fn connect(host: String) -> Result<Self, NodeClientError>;
    async fn get_block_number(&self) -> Result<BlockNumber, NodeClientError>;
    async fn get_block(&self, block: BlockNumber) -> Result<EthBlock, NodeClientError>;
    async fn get_transactions(
        &self,
        n: BlockNumber,
    ) -> Result<Vec<EthTransaction>, NodeClientError>;
    // TODO: move away from Filter
    async fn get_logs(&self, filter: &Criteria) -> Result<Vec<EthLog>, NodeClientError>;
}

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

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    async fn test_eth_get_block_number() {
        let client = Eth::new("wss://eth.llamarpc.com".to_string())
            .await
            .unwrap();

        assert!(client.get_block_number().await.is_ok());
    }

    #[tokio::test]
    async fn test_eth_get_block() {
        let client = Eth::new("wss://eth.llamarpc.com".to_string())
            .await
            .unwrap();

        let block = client.get_block(1911151).await;
        println!("{:#?}", block);
        assert!(block.is_ok());
    }

    #[tokio::test]
    async fn test_eth_get_transactions() {
        let client = Eth::new("wss://eth.llamarpc.com".to_string())
            .await
            .unwrap();

        let tx = client.get_transactions(1911151).await;
        println!("{:#?}", tx);
        assert!(tx.is_ok());
    }

    //#[tokio::test]
    //async fn test_eth_get_log() {
    //    let client = Eth::new("wss://eth.llamarpc.com".to_string())
    //        .await
    //        .unwrap();

    //    let filter = Filter::new().select(1911151..1911152);
    //    let criteria: Criteria = filter.into();
    //    let logs = client.get_logs(&filter).await;

    //    println!("{:#?}", logs);
    //    assert!(logs.is_ok());
    //}

    #[tokio::test]
    async fn test_eth_latest_block() {
        let client = Eth::new("wss://eth.llamarpc.com".to_string())
            .await
            .unwrap();

        let block = client
            .get_block(client.get_block_number().await.unwrap())
            .await;
        println!("{:#?}", block);
        assert!(block.is_ok());
    }

    #[tokio::test]
    async fn test_zksync_get_block_number() {
        let client = Eth::new("wss://sepolia.era.zksync.dev/ws".to_string())
            .await
            .unwrap();

        assert!(client.get_block_number().await.is_ok());
    }
}
