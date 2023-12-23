pub mod node;
pub mod storage;

use crate::node_provider;

#[cfg(all(feature = "eth", feature = "http"))]
node_provider!(EthHttp, ethers_providers::Provider<ethers_providers::Http>);

#[cfg(all(feature = "eth", feature = "ws"))]
node_provider!(EthWs, ethers_providers::Provider<ethers_providers::Ws>);

#[cfg(all(feature = "eth", feature = "ipc"))]
node_provider!(EthIpc, ethers_providers::Provider<ethers_providers::Ipc>);

//#[cfg(feature = "postgres")]
//storage_provider!(Postgres, sqlx::postgres::PgPool);
