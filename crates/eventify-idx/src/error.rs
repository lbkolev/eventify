#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Fetching block failed {0}")]
    FetchBlock(String),

    #[error("Fetching events failed {0}")]
    FetchLog(String),

    #[error("Missing transport node")]
    MissingTransportNode,

    #[error("Fetching block failed {0}")]
    FetchBlockNumberError(String),

    #[error("Failed to parse URL '{0}': {1}")]
    UrlParseError(String, String),

    #[error("Failed to create IPC transport with URL '{0}': {1}")]
    IpcTransportCreationError(String, String),

    #[error("Failed to create WebSocket transport with URL '{0}': {1}")]
    WsTransportCreationError(String, String),

    #[error("Missing transport storage")]
    MissingTransportStorage,

    #[error("{0}")]
    SubscriptionNewBlock(String),

    #[error("{0}")]
    SubscriptionNewLog(String),

    #[error(transparent)]
    JoinTask(#[from] tokio::task::JoinError),

    #[error(transparent)]
    EventifyPrimitives(#[from] eventify_primitives::Error),

    #[error(transparent)]
    Sql(#[from] sqlx::Error),

    #[error(transparent)]
    EthersCore(#[from] ethers_providers::ProviderError),

    #[error(transparent)]
    Migrate(#[from] sqlx::migrate::MigrateError),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Url(#[from] url::ParseError),

    #[error("{0}")]
    InvalidChain(String),

    #[error("{0}")]
    InvalidDatabase(String),
}

#[derive(Debug, Display, Error)]
#[non_exhaustive]
enum NodeProviderError {
    #[error("Failed to fetch block number")]
    FetchBlockNumber,

    #[error("Failed to fetch block")]
    FetchBlock,

    #[error("Failed to fetch transactions")]
    FetchTransactions,

    #[error("Failed to fetch logs")]
    FetchLogs,

}