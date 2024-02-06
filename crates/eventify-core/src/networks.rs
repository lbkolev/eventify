pub mod eth;

use std::num::ParseIntError;

#[derive(Clone)]
pub struct NetworkClient {
    inner: std::sync::Arc<reconnecting_jsonrpsee_ws_client::Client>,
    pub host: String,
}

impl NetworkClient {
    pub async fn new(host: String) -> Result<NetworkClient, NetworkError> {
        Ok(Self {
            inner: std::sync::Arc::new(
                reconnecting_jsonrpsee_ws_client::Client::builder()
                    .retry_policy(reconnecting_jsonrpsee_ws_client::FixedInterval::from_millis(500))
                    .enable_ws_ping(
                        reconnecting_jsonrpsee_ws_client::PingConfig::new()
                            .ping_interval(std::time::Duration::from_secs(6))
                            .inactive_limit(std::time::Duration::from_secs(30)),
                    )
                    .build(host.clone())
                    .await?,
            ),
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

#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum NetworkError {
    #[error("Failed to connect to node: {0}")]
    ConnectionFailed(#[from] jsonrpsee::core::ClientError),

    #[error("Failed to get the latest block number. {err}")]
    GetLatestBlockFailed { err: String },

    #[error("Failed to get block #{n}. {err}")]
    GetBlockFailed { n: u64, err: String },

    #[error("Failed to get transactions from block #{n}. {err}")]
    GetTransactionsFailed { n: u64, err: String },

    #[error("Failed to get logs. {err}")]
    GetLogsFailed { err: String },

    #[error("Failed to get block from sub {sub}, with params {params}. {err}")]
    BlockSubscriptionFailed {
        sub: String,
        params: String,
        err: String,
    },

    #[error("Failed to get log from sub {sub}, with params {params}. {err}")]
    LogSubscriptionFailed {
        sub: String,
        params: String,
        err: String,
    },

    #[error(transparent)]
    ParseInt(#[from] ParseIntError),

    #[error("Failed deserialization, {err}")]
    DeserializationFailed { err: String },
}
