#![doc = include_str!("../README.md")]

pub mod app;
pub mod collector;
pub mod error;
pub mod serve;

pub use app::App;
pub use error::Error;
pub use serve::run;

/// The Result used throughout the indexer
type Result<T> = std::result::Result<T, error::Error>;

#[derive(Debug)]
pub enum SupportedChains {
    Ethereum,
}
