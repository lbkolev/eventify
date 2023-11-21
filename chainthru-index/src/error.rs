#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Fetching block {0} failed")]
    FetchBlock(String),

    #[error("executing query {0}")]
    Sql(#[from] sqlx::Error),

    #[error("Web3 error: {0}")]
    Web3(#[from] web3::Error),

    #[error("executing database migrations: {0}")]
    Migrate(#[from] sqlx::migrate::MigrateError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("URL error: {0}")]
    Url(#[from] url::ParseError),
}
