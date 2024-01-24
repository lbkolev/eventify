pub mod database;
pub mod server;

pub mod configs {
    pub use crate::{
        database::DatabaseConfig,
        server::{ApplicationConfig, ServerConfig},
    };
}
