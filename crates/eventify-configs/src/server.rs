#[derive(Clone, Debug, serde::Deserialize)]
pub struct ServerConfig {
    pub database: crate::configs::DatabaseConfig,
    pub application: ApplicationConfig,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct ApplicationConfig {
    pub host: String,
    pub port: u16,

    /// The number of workers to start
    ///
    /// by default, the number of the machine's physical cores.
    pub worker_threads: usize,
}
