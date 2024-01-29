pub mod core;
pub mod database;
pub mod server;

pub mod configs {
    pub use crate::{
        core::{BlockRange, CollectorConfig, ManagerConfig},
        database::DatabaseConfig,
        server::{ApplicationConfig, ServerConfig},
    };
}
