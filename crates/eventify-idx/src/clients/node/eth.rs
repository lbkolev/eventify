use std::sync::Arc;

use crate::clients::node::{Auth, EthHttp, EthIpc, EthWs};

#[cfg(all(feature = "eth", feature = "http"))]
#[async_trait::async_trait]
impl Auth for EthHttp {
    async fn connect(url: &str) -> Self {
        Self {
            inner: Arc::new(
                ethers_providers::Provider::<ethers_providers::Http>::try_from(url)
                    .unwrap_or_else(|_| panic!("failed to connect to provider {}", url)),
            ),
        }
    }
}

#[cfg(all(feature = "eth", feature = "ws"))]
#[async_trait::async_trait]
impl Auth for EthWs {
    async fn connect(url: &str) -> Self {
        Self {
            inner: Arc::new(ethers_providers::Provider::new(
                ethers_providers::Ws::connect(url)
                    .await
                    .unwrap_or_else(|_| panic!("failed to connect to provider {}", url)),
            )),
        }
    }
}

#[cfg(all(feature = "eth", feature = "ipc"))]
#[async_trait::async_trait]
impl Auth for EthIpc {
    async fn connect(url: &str) -> Self {
        Self {
            inner: Arc::new(ethers_providers::Provider::new(
                ethers_providers::Ipc::connect(url)
                    .await
                    .unwrap_or_else(|_| panic!("failed to connect to provider {}", url)),
            )),
        }
    }
}
