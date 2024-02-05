use std::sync::Arc;

use crate::{impl_provider, provider::Node, NodeError};
use alloy_primitives::{BlockNumber, B256};
use eyre::Result;
use jsonrpsee::{
    core::client::{ClientT, Subscription, SubscriptionClientT},
    rpc_params,
    ws_client::{WsClient, WsClientBuilder},
};

use eventify_primitives::network::{
    Criteria, EthBlock, EthLog, EthTransaction, TransactionResponse,
};

impl_provider!(Eth, WsClient);
impl Eth {
    pub async fn new(host: String) -> Result<Self> {
        Self::connect_with_retry(host, 5).await
    }

    pub async fn connect(host: String) -> Result<Self> {
        Ok(Self {
            inner: Arc::new(WsClientBuilder::default().build(&host).await?),
        })
    }

    pub async fn connect_with_retry(host: String, max_retries: i32) -> Result<Self> {
        let mut retries = 0;
        loop {
            match Self::connect(host.clone()).await {
                Ok(client) => return Ok(client),
                Err(e) => {
                    if retries >= max_retries {
                        return Err(e);
                    } else {
                        retries += 1;
                    }
                }
            }
        }
    }
}

impl Node for Eth {
    async fn get_block_number(&self) -> Result<BlockNumber> {
        let s: Result<String> = self
            .inner
            .request("eth_blockNumber", rpc_params![])
            .await
            .map_err(|e| NodeError::GetLatestBlockFailed { err: e.to_string() }.into());

        match s {
            Ok(s) => Ok(BlockNumber::from_str_radix(s.trim_start_matches("0x"), 16)?),
            Err(e) => Err(e),
        }
    }

    async fn get_block(&self, n: BlockNumber) -> Result<EthBlock<B256>> {
        self.inner
            .request(
                "eth_getBlockByNumber",
                rpc_params![format!("0x{:x}", n), false],
            )
            .await
            .map_err(|e| {
                NodeError::GetBlockFailed {
                    n,
                    err: e.to_string(),
                }
                .into()
            })
    }

    async fn get_transactions(&self, n: BlockNumber) -> Result<Vec<EthTransaction>> {
        let r: Result<TransactionResponse> = self
            .inner
            .request(
                "eth_getBlockByNumber",
                rpc_params![format!("0x{:x}", n), true],
            )
            .await
            .map_err(|e| {
                NodeError::GetTransactionsFailed {
                    n,
                    err: e.to_string(),
                }
                .into()
            });

        match r {
            Ok(r) => Ok(r.transactions),
            Err(e) => Err(e),
        }
    }

    async fn get_logs(&self, filter: &Criteria) -> Result<Vec<EthLog>> {
        self.inner
            .request("eth_getLogs", rpc_params!(filter))
            .await
            .map_err(|e| NodeError::GetLogsFailed { err: e.to_string() }.into())
    }

    async fn stream_blocks(&self) -> Result<Subscription<EthBlock<B256>>> {
        self.inner
            .subscribe("eth_subscribe", rpc_params!["newHeads"], "eth_unsubscribe")
            .await
            .map_err(|e| {
                NodeError::BlockSubscriptionFailed {
                    sub: "eth_subscribe".into(),
                    params: "newHeads".into(),
                    err: e.to_string(),
                }
                .into()
            })
    }

    async fn stream_logs(&self) -> Result<Subscription<EthLog>> {
        self.inner
            .subscribe("eth_subscribe", rpc_params!["logs"], "eth_unsubscribe")
            .await
            .map_err(|e| {
                NodeError::LogSubscriptionFailed {
                    sub: "eth_subscribe".into(),
                    params: "logs".into(),
                    err: e.to_string(),
                }
                .into()
            })
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
}
