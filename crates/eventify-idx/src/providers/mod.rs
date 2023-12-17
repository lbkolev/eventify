pub mod eth;

use crate::{provider_struct, types::provider::NodeProvider};

// --- eth
#[cfg(all(feature = "eth", feature = "http"))]
provider_struct!(EthHttp, ethers_providers::Http);

#[cfg(all(feature = "eth", feature = "ws"))]
provider_struct!(EthWs, ethers_providers::Ws);

#[cfg(all(feature = "eth", feature = "ipc"))]
provider_struct!(EthIpc, ethers_providers::Ipc);
// ---
