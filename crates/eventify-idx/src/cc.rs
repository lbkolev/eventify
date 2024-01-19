use std::sync::Arc;

use crate::{clients::NodeClient, NodeClientError};
use alloy_primitives::{Address, BlockNumber, Bytes, B256, U64};
use eventify_primitives::{Criteria, EthBlock, EthTransaction, TransactionResponse};
use jsonrpsee::{
    core::client::ClientT,
    rpc_params,
    ws_client::{WsClient, WsClientBuilder},
};
use serde::{Deserialize, Deserializer};

#[derive(Clone, Debug)]
pub struct Eth {
    inner: Arc<WsClient>,
}

impl Eth {
    pub async fn new(host: String) -> Result<Self, NodeClientError> {
        Self::connect(host).await
    }
}

#[async_trait::async_trait]
impl NodeClient for Eth {
    async fn connect(host: String) -> Result<Self, NodeClientError> {
        Ok(Self {
            inner: Arc::new(
                WsClientBuilder::default()
                    .build(&host)
                    .await
                    .map_err(|_| NodeClientError::Connect)?,
            ),
        })
    }

    async fn get_block_number(&self) -> Result<BlockNumber, NodeClientError> {
        let s: Result<String, NodeClientError> = self
            .inner
            .request("eth_blockNumber", rpc_params![])
            .await
            .map_err(|_| NodeClientError::LatestBlock);

        if let Ok(s) = s {
            Ok(BlockNumber::from_str_radix(s.trim_start_matches("0x"), 16)?)
        } else {
            Err(NodeClientError::LatestBlock)
        }
    }

    // returns block without transactions
    async fn get_block(&self, n: BlockNumber) -> Result<EthBlock, NodeClientError> {
        self.inner
            .request(
                "eth_getBlockByNumber",
                rpc_params![format!("0x{:x}", n), false],
            )
            .await
            .map_err(|_| NodeClientError::Block(n))
    }

    async fn get_transactions(
        &self,
        n: BlockNumber,
    ) -> Result<Vec<EthTransaction>, NodeClientError> {
        let r: TransactionResponse = self
            .inner
            .request(
                "eth_getBlockByNumber",
                rpc_params![format!("0x{:x}", n), true],
            )
            .await
            .map_err(|_| NodeClientError::Transactions(n))?;

        Ok(r.transactions)
    }

    async fn get_logs(&self, filter: &Criteria) -> Result<Vec<EthLog>, NodeClientError> {
        self.inner
            .request("eth_getLogs", rpc_params!(filter))
            .await
            .map_err(|_| NodeClientError::Logs("tmp".to_string()))
    }
}

#[derive(Clone, Debug)]
pub struct Zksync {
    inner: Arc<WsClient>,
    host: String,
}

impl Zksync {
    pub async fn new(host: String) -> Result<Self, NodeClientError> {
        Ok(Self {
            inner: Arc::new(
                WsClientBuilder::default()
                    .build(&host)
                    .await
                    .map_err(|_| NodeClientError::Connect)?,
            ),
            host,
        })
    }

    async fn get_block_number(&self) -> Result<BlockNumber, NodeClientError> {
        let s: Result<String, NodeClientError> = self
            .inner
            .request("eth_blockNumber", rpc_params![])
            .await
            .map_err(|_| NodeClientError::LatestBlock);

        if let Ok(s) = s {
            Ok(BlockNumber::from_str_radix(s.trim_start_matches("0x"), 16)?)
        } else {
            Err(NodeClientError::LatestBlock)
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
