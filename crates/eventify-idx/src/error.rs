#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    NodeClient(#[from] NodeClientError),

    #[error("Failed to parse URL '{0}': {1}")]
    UrlParseError(String, String),

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
    InvalidChainKind(String),

    #[error("{0}")]
    InvalidDatabase(String),
}

#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum NodeClientError {
    #[error("failed to connect to node")]
    Connect,

    #[error("failed to get the latest block number")]
    GetLatestBlock,

    #[error("failed to get block {0}")]
    GetBlock(u64),

    #[error("failed to get transactions from block {0}")]
    GetTransactions(u64),

    #[error("Failed to get logs for criteria {0}")]
    GetLogs(String),
}
