pub mod api;
pub mod error;
pub mod startup;

/// The result type used through the server application code.
type Result<T> = std::result::Result<T, crate::error::Error>;

#[derive(Clone, Debug, serde::Deserialize)]
pub struct ApplicationSettings {
    pub host: String,
    pub port: u16,

    /// The number of workers to start
    ///
    /// by default, the number of the machine's physical cores.
    pub worker_threads: usize,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct Settings {
    pub database: chainthru_types::DatabaseSettings,
    pub application: ApplicationSettings,
}

/// The entry point of the API server.
pub async fn run(settings: Settings) -> Result<()> {
    let application = startup::Application::build(settings).await?;
    application.run_until_stopped().await?;

    Ok(())
}
