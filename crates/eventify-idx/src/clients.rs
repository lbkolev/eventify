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

//#[derive(Debug, Default)]
//pub struct NodeProvider<C>
//where
//    C: NodeClient<crate::Error>,
//{
//    client: C,
//}
//
//impl<C> NodeProvider<C>
//where
//    C: NodeClient<crate::Error>,
//{
//    pub fn new(client: C) -> Self {
//        Self { client }
//    }
//
//    pub fn client(&self) -> &C {
//        &self.client
//    }
//
//    pub fn client_mut(&mut self) -> &mut C {
//        &mut self.client
//    }
//
//    pub fn into_client(self) -> C {
//        self.client
//    }
//
//    //pub async fn connect(url: &str) -> Result<Self> {
//    //    let client = C::connect(url).await?;
//
//    //    Ok(Self::new(client))
//    //}
//
//    pub async fn connect(url: &str) -> Result<Self> {
//        let client = match Url::parse(url)?.scheme() {
//            "http" => {
//                #[cfg(feature = "http")]
//                {
//                    Self::new(EthHttp::connect(url).await?)
//                }
//                #[cfg(not(feature = "http"))]
//                {
//                    return Err(crate::Error::new("http feature is not enabled"));
//                }
//            }
//            "ws" => {
//                #[cfg(feature = "ws")]
//                {
//                    EthWs::connect(url).await?
//                }
//                #[cfg(not(feature = "ws"))]
//                {
//                    return Err(crate::Error::new("ws feature is not enabled"));
//                }
//            }
//            "ipc" => {
//                #[cfg(feature = "ipc")]
//                {
//                    EthIpc::connect(url).await?
//                }
//                #[cfg(not(feature = "ipc"))]
//                {
//                    return Err(crate::Error::new("ipc feature is not enabled"));
//                }
//            }
//            _ => return Err(crate::Error::new("Invalid node url")),
//        };
//
//        // Assuming C can be constructed from EthHttp, EthWs, EthIpc
//        C::from_client(client).map(Self::new)
//    }
//}
//
//impl<C> Deref for NodeProvider<C>
//where
//    C: NodeClient<crate::Error>,
//{
//    type Target = C;
//
//    fn deref(&self) -> &Self::Target {
//        &self.client
//    }
//}
//
//impl<C> DerefMut for NodeProvider<C>
//where
//    C: NodeClient<crate::Error>,
//{
//    fn deref_mut(&mut self) -> &mut Self::Target {
//        &mut self.client
//    }
//}
//
