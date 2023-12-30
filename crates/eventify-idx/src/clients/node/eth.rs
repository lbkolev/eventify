use std::{error::Error, sync::Arc};

use alloy_primitives::BlockNumber;
use ethers_core::types::{BlockId, Filter};
use ethers_providers::Middleware;

use crate::{
    clients::{EthHttp, EthIpc, EthWs},
    error::NodeClientError,
    types::NodeClient,
};
use eventify_primitives::{Block, Criteria, Log, Transaction};

#[cfg(all(feature = "eth", feature = "http"))]
#[async_trait::async_trait]
impl NodeClient for EthHttp {
    async fn new(url: &str) -> Result<Self, NodeClientError> {
        Self::connect(url).await
    }

    async fn connect(url: &str) -> Result<Self, NodeClientError> {
        let provider_result = ethers_providers::Provider::<ethers_providers::Http>::try_from(url);

        match provider_result {
            Ok(provider) => Ok(Self {
                inner: Arc::new(provider),
            }),
            Err(_) => Err(NodeClientError::Connect),
        }
    }

    async fn get_block_number(&self) -> Result<BlockNumber, NodeClientError> {
        self.inner
            .get_block_number()
            .await
            .map(|n| n.as_u64())
            .map_err(|_| NodeClientError::GetLatestBlock)
    }

    async fn get_block(&self, block: BlockNumber) -> Result<Block, NodeClientError> {
        // todo: use .get_block() instead of .get_block_with_txs()
        self.inner
            .get_block_with_txs(BlockId::Number(block.into()))
            .await
            .map_err(|_| NodeClientError::GetBlock(block))?
            .map(Block::from)
            .ok_or(NodeClientError::GetBlock(block))
    }

    async fn get_transactions(
        &self,
        block: BlockNumber,
    ) -> Result<Vec<Transaction>, NodeClientError> {
        Ok(self
            .inner
            .get_block_with_txs(BlockId::Number(block.into()))
            .await
            .map_err(|_| NodeClientError::GetTransactions(block))?
            .ok_or(NodeClientError::GetTransactions(block))?
            .transactions
            .into_iter()
            .map(Transaction::from)
            .collect())
    }

    async fn get_logs(&self, criteria: &Criteria) -> Result<Vec<Log>, NodeClientError> {
        Ok(self
            .inner
            .get_logs(&Filter::from(criteria))
            .await
            .map_err(|_| NodeClientError::GetLogs(criteria.clone()))?
            .into_iter()
            .map(Log::from)
            .collect())
    }
}

#[cfg(all(feature = "eth", feature = "ws"))]
#[async_trait::async_trait]
impl NodeClient for EthWs {
    async fn new(url: &str) -> Result<Self, NodeClientError> {
        Self::connect(url).await
    }

    async fn connect(url: &str) -> Result<Self, NodeClientError> {
        let provider = ethers_providers::Ws::connect(url).await;

        match provider {
            Ok(provider) => Ok(Self {
                inner: Arc::new(ethers_providers::Provider::new(provider)),
            }),
            Err(_) => Err(NodeClientError::Connect),
        }
    }

    async fn get_block_number(&self) -> Result<BlockNumber, NodeClientError> {
        self.inner
            .get_block_number()
            .await
            .map(|n| n.as_u64())
            .map_err(|_| NodeClientError::GetLatestBlock)
    }

    async fn get_block(&self, block: BlockNumber) -> Result<Block, NodeClientError> {
        // todo: use .get_block() instead of .get_block_with_txs()
        self.inner
            .get_block_with_txs(BlockId::Number(block.into()))
            .await
            .map_err(|_| NodeClientError::GetBlock(block))?
            .map(Block::from)
            .ok_or(NodeClientError::GetBlock(block))
    }

    async fn get_transactions(
        &self,
        block: BlockNumber,
    ) -> Result<Vec<Transaction>, NodeClientError> {
        Ok(self
            .inner
            .get_block_with_txs(BlockId::Number(block.into()))
            .await
            .map_err(|_| NodeClientError::GetTransactions(block))?
            .ok_or(NodeClientError::GetTransactions(block))?
            .transactions
            .into_iter()
            .map(Transaction::from)
            .collect())
    }

    async fn get_logs(&self, criteria: &Criteria) -> Result<Vec<Log>, NodeClientError> {
        Ok(self
            .inner
            .get_logs(&Filter::from(criteria))
            .await
            .map_err(|_| NodeClientError::GetLogs(criteria.clone()))?
            .into_iter()
            .map(Log::from)
            .collect())
    }
}

#[cfg(all(feature = "eth", feature = "ipc"))]
#[async_trait::async_trait]
impl NodeClient for EthIpc {
    async fn new(url: &str) -> Result<Self, NodeClientError> {
        Self::connect(url).await
    }

    async fn connect(url: &str) -> Result<Self, NodeClientError> {
        let provider = ethers_providers::Ipc::connect(url).await;

        match provider {
            Ok(provider) => Ok(Self {
                inner: Arc::new(ethers_providers::Provider::new(provider)),
            }),
            Err(_) => Err(NodeClientError::Connect),
        }
    }

    async fn get_block_number(&self) -> Result<BlockNumber, NodeClientError> {
        self.inner
            .get_block_number()
            .await
            .map(|n| n.as_u64())
            .map_err(|_| NodeClientError::GetLatestBlock)
    }

    async fn get_block(&self, block: BlockNumber) -> Result<Block, NodeClientError> {
        // todo: use .get_block() instead of .get_block_with_txs()
        self.inner
            .get_block_with_txs(BlockId::Number(block.into()))
            .await
            .map_err(|_| NodeClientError::GetBlock(block))?
            .map(Block::from)
            .ok_or(NodeClientError::GetBlock(block))
    }

    async fn get_transactions(
        &self,
        block: BlockNumber,
    ) -> Result<Vec<Transaction>, NodeClientError> {
        Ok(self
            .inner
            .get_block_with_txs(BlockId::Number(block.into()))
            .await
            .map_err(|_| NodeClientError::GetTransactions(block))?
            .ok_or(NodeClientError::GetTransactions(block))?
            .transactions
            .into_iter()
            .map(Transaction::from)
            .collect())
    }

    async fn get_logs(&self, criteria: &Criteria) -> Result<Vec<Log>, NodeClientError> {
        Ok(self
            .inner
            .get_logs(&Filter::from(criteria))
            .await
            .map_err(|_| NodeClientError::GetLogs(criteria.clone()))?
            .into_iter()
            .map(Log::from)
            .collect())
    }
}
