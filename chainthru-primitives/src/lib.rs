//! Primitive types used throughout the chainthru application crates.
#![allow(clippy::option_map_unit_fn)]

pub mod block;
pub mod database;
pub mod erc20;
pub mod erc721;
pub mod error;
pub mod macros;
pub mod transaction;

pub use block::IndexedBlock;
pub use database::Settings as DatabaseSettings;
pub use transaction::{IndexedTransaction, TransactionBoilerplate};

use sqlx::PgPool;

/// The result type used through the application code.
type Result<T> = std::result::Result<T, error::Error>;

#[async_trait::async_trait]
pub trait Insertable: Sized {
    async fn insert(&self, conn: &PgPool) -> Result<()>;
}
