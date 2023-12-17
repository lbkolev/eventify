use crate::{provider_struct, types::provider::NodeProvider};
use eventify_primitives::Criteria;

use crate::providers::EthHttp;

use ethers_core::types::{Block, Transaction};
use ethers_providers::{JsonRpcClient, Middleware, PubsubClient};

//#[async_trait::async_trait]
//impl<T: JsonRpcClient + Middleware + Clone> NodeProvider for T {}

//#[async_trait::async_trait]
//impl NodeProvider for ethers_providers::Ipc {}
//
//#[async_trait::async_trait]
//impl NodeProvider for ethers_providers::Ws {}
//
//#[async_trait::async_trait]
//impl NodeProvider for ethers_providers::Http {}


#[async_trait::async_trait]
impl NodeProvider for EthHttp {
    type Error = ethers_providers::ProviderError;

    async fn get_block_number(&self) -> Result<u64, Self::Error> {
        self.inner.get_block_number().await.map(|n| n.as_u64())
    }

    async fn get_block(&self, block: u64) -> Result<Block<()>, Self::Error> {
        //self.inner.get_block(block).await
        todo!()
    }

    async fn get_transactions(&self, block: u64) -> Result<Vec<Transaction>, Self::Error> {
        todo!()
    }

    async fn get_logs(&self, criteria: Criteria) -> Result<(), Self::Error> {
        todo!()
    }

    async fn stream_blocks(&self) -> Result<(), Self::Error> {
        todo!()
    }

    async fn stream_transactions(&self) -> Result<(), Self::Error> {
        todo!()
    }

    async fn stream_logs(&self, criteria: Criteria) -> Result<(), Self::Error> {
        todo!()
    }
}

//#[async_trait::async_trait]
//impl Provider for ethers_providers::Provider<ethers_providers::Http> {
//    type Error = ethers_providers::ProviderError;
//
//    async fn get_latest_block(&self) -> Result<u64, Self::Error> {
//        self..get_block_number().await.map(|n| n.as_u64())
//    }
//
//    async fn fetch_block(&self, block_number: u64) -> Result<Block<Transaction>, Self::Error> {
//        self.get_block(block_number).await
//    }
//
//    async fn fetch_block_number(&self) -> Result<u64, Self::Error> {
//        self.get_block_number().await.map(|n| n.as_u64())
//    }
//}
