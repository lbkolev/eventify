pub mod node;
pub mod storage;

use crate::provider;

#[cfg(all(feature = "eth", feature = "http"))]
provider!(EthHttp, ethers_providers::Provider<ethers_providers::Http>);

#[cfg(all(feature = "eth", feature = "ws"))]
provider!(EthWs, ethers_providers::Provider<ethers_providers::Ws>);

#[cfg(all(feature = "eth", feature = "ipc"))]
provider!(EthIpc, ethers_providers::Provider<ethers_providers::Ipc>);
