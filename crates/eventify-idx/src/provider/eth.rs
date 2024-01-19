use std::sync::Arc;

use crate::{provider::NodeClient, NodeClientError};
use alloy_primitives::BlockNumber;
use eventify_primitives::{Criteria, EthBlock, EthLog, EthTransaction, TransactionResponse};
use jsonrpsee::{
    core::client::ClientT,
    rpc_params,
    ws_client::{WsClient, WsClientBuilder},
};

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
            .map_err(|_| NodeClientError::Logs("".to_string()))
    }
}
