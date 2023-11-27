use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to bind address")]
    BindAddress(#[from] std::io::Error),

    #[error("Failed to connect to database")]
    ConnectToDatabase(#[from] sqlx::Error),

    #[error("Failed to run database migrations")]
    RunMigrations(#[from] sqlx::migrate::MigrateError),
}
