pub mod ethereum;

use std::sync::Arc;

use jsonrpsee::{core::client::Error as RpcError, ws_client::WsClientBuilder};

#[derive(Clone)]
pub struct NetworkClient {
    inner: Arc<jsonrpsee::ws_client::WsClient>,
    pub host: String,
}

impl NetworkClient {
    pub async fn new(host: String) -> Result<NetworkClient, RpcError> {
        Ok(Self {
            inner: Arc::new(WsClientBuilder::default().build(&host).await?),
            host,
        })
    }
}

impl std::fmt::Display for NetworkClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", stringify!(NetworkClient))
    }
}

impl std::fmt::Debug for NetworkClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", stringify!(NetworkClient))
    }
}
