#[macro_export]
macro_rules! node_client {
    ($name:ident, $transport:ty) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            pub inner: std::sync::Arc<$transport>,
        }

        impl std::ops::Deref for $name {
            type Target = $transport;

            fn deref(&self) -> &Self::Target {
                &self.inner
            }
        }

        impl From<$transport> for $name {
            fn from(inner: $transport) -> Self {
                Self {
                    inner: std::sync::Arc::new(inner),
                }
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}: [{}]", stringify!($name), stringify!($transport))
            }
        }

        impl $name {
            pub fn inner(&self) -> &$transport {
                &self.inner
            }

            pub fn with_inner(&self, inner: $transport) -> Self {
                Self {
                    inner: std::sync::Arc::new(inner),
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_eth {
    ($name:ident) => {
        impl $name {
            pub async fn new(url: &str) -> Self {
                Self::connect(url).await
            }
        }

        #[async_trait::async_trait]
        impl $crate::clients::NodeClient for $name {
            async fn get_block_number(
                &self,
            ) -> Result<alloy_primitives::BlockNumber, $crate::error::NodeClientError> {
                self.inner
                    .get_block_number()
                    .await
                    .map(|n| n.as_u64())
                    .map_err(|_| $crate::error::NodeClientError::GetLatestBlock)
            }

            async fn get_block(
                &self,
                block: alloy_primitives::BlockNumber,
            ) -> Result<eventify_primitives::Block, $crate::error::NodeClientError> {
                // todo: use .get_block() instead of .get_block_with_txs()
                self.inner
                    .get_block_with_txs(ethers_core::types::BlockId::Number(block.into()))
                    .await
                    .map_err(|_| $crate::error::NodeClientError::GetBlock(block))?
                    .map(eventify_primitives::Block::from)
                    .ok_or($crate::error::NodeClientError::GetBlock(block))
            }

            async fn get_transactions(
                &self,
                block: alloy_primitives::BlockNumber,
            ) -> Result<Vec<eventify_primitives::Transaction>, $crate::error::NodeClientError> {
                Ok(self
                    .inner
                    .get_block_with_txs(ethers_core::types::BlockId::Number(block.into()))
                    .await
                    .map_err(|_| $crate::error::NodeClientError::GetTransactions(block))?
                    .ok_or($crate::error::NodeClientError::GetTransactions(block))?
                    .transactions
                    .into_iter()
                    .map(eventify_primitives::Transaction::from)
                    .collect())
            }

            async fn get_logs(
                &self,
                criteria: &eventify_primitives::Criteria,
            ) -> Result<Vec<eventify_primitives::Log>, $crate::error::NodeClientError> {
                Ok(self
                    .inner
                    .get_logs(&ethers_core::types::Filter::from(criteria))
                    .await
                    .map_err(|_| $crate::error::NodeClientError::GetLogs(criteria.name.clone()))?
                    .into_iter()
                    .map(eventify_primitives::Log::from)
                    .collect())
            }
        }
    };
}
