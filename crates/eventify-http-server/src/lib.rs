#![doc = include_str!("../README.md")]
#![warn(missing_debug_implementations, unreachable_pub, rustdoc::all)]
#![deny(unused_must_use, rust_2018_idioms)]

pub mod api;
pub mod error;
pub mod startup;
pub mod types;

pub use error::Error;

/// The result type used through the server application code.
type Result<T> = std::result::Result<T, crate::error::Error>;

/// The entry point of the API server.
pub async fn run(
    config: eventify_configs::configs::ApplicationConfig,
    pool: sqlx::pool::Pool<sqlx::Postgres>,
) -> Result<()> {
    let application = startup::Application::build(config, pool).await?;
    application.run_until_stopped().await?;

    Ok(())
}
