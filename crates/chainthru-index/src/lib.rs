#![doc = include_str!("../README.md")]

pub mod app;
pub mod collector;
pub mod error;
pub mod manager;
pub mod processor;
pub mod serve;
pub mod types;

pub use app::App;
pub use error::Error;
pub use manager::Manager;
pub use processor::Processor;
pub use serve::{run, run_par};
pub use types::{block_processor::BlockProcessor, log_processor::LogProcessor, runner::Runner, *};

/// The Result used throughout the indexer
type Result<T> = std::result::Result<T, error::Error>;

#[derive(Debug)]
pub enum SupportedChains {
    Ethereum,
}
