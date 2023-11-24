#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Fetching block {0} failed")]
    FetchBlock(String),

    #[error(transparent)]
    Sql(#[from] sqlx::Error),

    #[error(transparent)]
    Web3(#[from] web3::Error),

    //#[error(transparent)]
    //EthersCore(#[from] ethers_core::abi::Error),
    #[error("executing database migrations: {0}")]
    Migrate(#[from] sqlx::migrate::MigrateError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("URL error: {0}")]
    Url(#[from] url::ParseError),
}
