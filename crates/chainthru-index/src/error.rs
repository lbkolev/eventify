#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Fetching block {0} failed")]
    FetchBlock(String),

    #[error("Fetching events {0} failed")]
    FetchEvent(String),

    #[error(transparent)]
    ChainthruPrimitives(#[from] chainthru_primitives::Error),

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
}
