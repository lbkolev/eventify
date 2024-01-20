use std::sync::Arc;

use crate::{impl_provider, provider::NodeProvider, NodeProviderError};
use alloy_primitives::{BlockNumber, B256};
use eventify_primitives::{Criteria, EthBlock, EthLog, EthTransaction, TransactionResponse};
use eyre::Result;
use jsonrpsee::{
    core::client::{ClientT, Subscription, SubscriptionClientT},
    rpc_params,
    ws_client::{WsClient, WsClientBuilder},
};

impl_provider!(Eth, WsClient);
impl Eth {
    pub async fn new(host: String) -> Result<Self> {
        Self::connect(host).await
    }
}

impl NodeProvider for Eth {
    async fn connect(host: String) -> Result<Self> {
        Ok(Self {
            inner: Arc::new(WsClientBuilder::default().build(&host).await?),
        })
    }

    async fn get_block_number(&self) -> Result<BlockNumber> {
        let s: Result<String> = self
            .inner
            .request("eth_blockNumber", rpc_params![])
            .await
            .map_err(|_| NodeProviderError::GetLatestBlockFailed.into());

        match s {
            Ok(s) => Ok(BlockNumber::from_str_radix(s.trim_start_matches("0x"), 16)?),
            Err(e) => Err(e),
        }
    }

    async fn get_block(&self, n: BlockNumber) -> Result<EthBlock> {
        self.inner
            .request(
                "eth_getBlockByNumber",
                rpc_params![format!("0x{:x}", n), false],
            )
            .await
            .map_err(|_| NodeProviderError::GetBlockFailed(n).into())
    }

    async fn get_transactions(&self, n: BlockNumber) -> Result<Vec<EthTransaction>> {
        let r: Result<TransactionResponse> = self
            .inner
            .request(
                "eth_getBlockByNumber",
                rpc_params![format!("0x{:x}", n), true],
            )
            .await
            .map_err(|_| NodeProviderError::GetTransactionsFailed(n).into());

        match r {
            Ok(r) => Ok(r.transactions),
            Err(e) => Err(e),
        }
    }

    // TODO:
    async fn get_logs(&self, filter: &Criteria) -> Result<Vec<EthLog>> {
        self.inner
            .request("eth_getLogs", rpc_params!(filter))
            .await
            .map_err(|_| NodeProviderError::Logs("".to_string()).into())
    }

    async fn stream_blocks(&self) -> Result<Subscription<EthBlock>> {
        self.inner
            .subscribe("eth_subscribe", rpc_params!["newHeads"], "eth_unsubscribe")
            .await
            .map_err(|_| {
                NodeProviderError::BlockSubscriptionFailed(
                    "eth_subscribe".to_string(),
                    "newHeads".to_string(),
                )
                .into()
            })
    }

    async fn stream_transactions(&self) -> Result<Subscription<B256>> {
        self.inner
            .subscribe(
                "eth_subscribe",
                rpc_params!["newPendingTransactions"],
                "eth_unsubscribe",
            )
            .await
            .map_err(|_| {
                NodeProviderError::TransactionSubscriptionFailed(
                    "eth_subscribe".to_string(),
                    "newPendingTransactions".to_string(),
                )
                .into()
            })
    }

    async fn stream_logs(&self) -> Result<Subscription<EthLog>> {
        self.inner
            .subscribe("eth_subscribe", rpc_params!["logs"], "eth_unsubscribe")
            .await
            .map_err(|_| {
                NodeProviderError::LogSubscriptionFailed(
                    "eth_subscribe".to_string(),
                    "logs".to_string(),
                )
                .into()
            })
    }
}
