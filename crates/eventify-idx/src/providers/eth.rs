use alloy_primitives::BlockNumber;
use ethers_core::types::{BlockId, Filter};
use ethers_providers::Middleware;

use crate::{
    providers::{EthHttp, EthIpc, EthWs},
    types::provider::NodeProvider,
};
use eventify_primitives::{Block, Criteria, Log, Transaction};

#[cfg(all(feature = "eth", feature = "http"))]
#[async_trait::async_trait]
impl NodeProvider<crate::Error> for EthHttp {
    async fn new(url: &str) -> Result<Self, crate::Error> {
        Self::connect(url).await
    }

    async fn connect(url: &str) -> Result<Self, crate::Error> {
        Ok(Self {
            inner: ethers_providers::Provider::<ethers_providers::Http>::try_from(url)?,
        })
    }

    async fn get_block_number(&self) -> Result<BlockNumber, crate::Error> {
        self.inner
            .get_block_number()
            .await
            .map(|n| n.as_u64())
            .map_err(|e| e.into())
    }

    async fn get_block(&self, block: BlockNumber) -> Result<Block, crate::Error> {
        self.inner
            .get_block_with_txs(BlockId::Number(block.into()))
            .await
            .map_err(|e| crate::Error::FetchBlock(format!("{}", e)))?
            .map(Block::from)
            .ok_or(crate::Error::FetchBlock("Block not found".to_string()))
    }

    async fn get_transactions(&self, block: BlockNumber) -> Result<Vec<Transaction>, crate::Error> {
        Ok(self
            .inner
            .get_block_with_txs(BlockId::Number(block.into()))
            .await
            .map_err(|e| crate::Error::FetchBlock(format!("{}", e)))?
            .ok_or(crate::Error::FetchBlock("Block not found".to_string()))?
            .transactions
            .into_iter()
            .map(Transaction::from)
            .collect())
    }

    async fn get_logs(&self, criteria: Criteria) -> Result<Vec<Log>, crate::Error> {
        Ok(self
            .inner
            .get_logs(&Filter::from(&criteria))
            .await
            .map_err(|e| crate::Error::FetchLog(format!("Failed to fetch logs: {}", e)))?
            .into_iter()
            .map(Log::from)
            .collect::<Vec<Log>>())
    }
}

#[cfg(all(feature = "eth", feature = "ws"))]
impl EthWs {}

#[cfg(all(feature = "eth", feature = "ws"))]
#[async_trait::async_trait]
impl NodeProvider<crate::Error> for EthWs {
    async fn new(url: &str) -> Result<Self, crate::Error> {
        Self::connect(url).await
    }

    async fn connect(url: &str) -> Result<Self, crate::Error> {
        Ok(Self {
            inner: ethers_providers::Provider::new(
                ethers_providers::Ws::connect(url).await.map_err(|e| {
                    crate::Error::WsTransportCreationError(url.to_string(), e.to_string())
                })?,
            ),
        })
    }

    async fn get_block_number(&self) -> Result<BlockNumber, crate::Error> {
        self.inner
            .get_block_number()
            .await
            .map(|n| n.as_u64())
            .map_err(|e| e.into())
    }

    async fn get_block(&self, block: BlockNumber) -> Result<Block, crate::Error> {
        self.inner
            .get_block_with_txs(BlockId::Number(block.into()))
            .await
            .map_err(|e| crate::Error::FetchBlock(format!("{}", e)))?
            .map(Block::from)
            .ok_or(crate::Error::FetchBlock("Block not found".to_string()))
    }

    async fn get_transactions(&self, block: BlockNumber) -> Result<Vec<Transaction>, crate::Error> {
        Ok(self
            .inner
            .get_block_with_txs(BlockId::Number(block.into()))
            .await
            .map_err(|e| crate::Error::FetchBlock(format!("{}", e)))?
            .ok_or(crate::Error::FetchBlock("Block not found".to_string()))?
            .transactions
            .into_iter()
            .map(Transaction::from)
            .collect())
    }

    async fn get_logs(&self, criteria: Criteria) -> Result<Vec<Log>, crate::Error> {
        Ok(self
            .inner
            .get_logs(&Filter::from(&criteria))
            .await
            .map_err(|e| crate::Error::FetchLog(format!("Failed to fetch logs: {}", e)))?
            .into_iter()
            .map(Log::from)
            .collect::<Vec<Log>>())
    }
}

#[cfg(all(feature = "eth", feature = "ipc"))]
#[async_trait::async_trait]
impl NodeProvider<crate::Error> for EthIpc {
    async fn new(url: &str) -> Result<Self, crate::Error> {
        Self::connect(url).await
    }

    async fn connect(url: &str) -> Result<Self, crate::Error> {
        Ok(Self {
            inner: ethers_providers::Provider::new(
                ethers_providers::Ipc::connect(url).await.map_err(|e| {
                    crate::Error::IpcTransportCreationError(url.to_string(), e.to_string())
                })?,
            ),
        })
    }

    async fn get_block_number(&self) -> Result<BlockNumber, crate::Error> {
        self.inner
            .get_block_number()
            .await
            .map(|n| n.as_u64())
            .map_err(|e| e.into())
    }

    async fn get_block(&self, block: BlockNumber) -> Result<Block, crate::Error> {
        self.inner
            .get_block_with_txs(BlockId::Number(block.into()))
            .await
            .map_err(|e| crate::Error::FetchBlock(format!("{}", e)))?
            .map(Block::from)
            .ok_or(crate::Error::FetchBlock("Block not found".to_string()))
    }

    async fn get_transactions(&self, block: BlockNumber) -> Result<Vec<Transaction>, crate::Error> {
        Ok(self
            .inner
            .get_block_with_txs(BlockId::Number(block.into()))
            .await
            .map_err(|e| crate::Error::FetchBlock(format!("{}", e)))?
            .ok_or(crate::Error::FetchBlock("Block not found".to_string()))?
            .transactions
            .into_iter()
            .map(Transaction::from)
            .collect())
    }

    async fn get_logs(&self, criteria: Criteria) -> Result<Vec<Log>, crate::Error> {
        Ok(self
            .inner
            .get_logs(&Filter::from(&criteria))
            .await
            .map_err(|e| crate::Error::FetchLog(format!("Failed to fetch logs: {}", e)))?
            .into_iter()
            .map(Log::from)
            .collect::<Vec<Log>>())
    }
}
