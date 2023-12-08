#![doc = include_str!("../README.md")]
#![allow(async_fn_in_trait)]

pub mod app;
pub mod error;
pub mod macros;
pub mod manager;
pub mod processor;
pub mod types;

pub use app::App;
pub use error::Error;
pub use macros::{BlockProcessor, LogProcessor};
pub use manager::Manager;
pub use processor::Processor;
pub use types::runner::Runner;

/// The Result used throughout the indexer
type Result<T> = std::result::Result<T, error::Error>;

#[derive(Debug)]
pub enum SupportedChains {
    Ethereum,
}
