#![doc = include_str!("../README.md")]

/// Re-export of the Chainthru Ethereum Indexer.
/// ...
pub use chainthru_index;

/// Re-Export of the Chainthru API server.
/// ...
pub use chainthru_server;

/// Re-export of the possible Errs.
/// ...
pub use error::Error;

pub mod error;
pub mod settings;

pub type Result<T> = std::result::Result<T, Error>;
