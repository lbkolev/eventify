//! Primitive types used throughout the chainthru application crates.
#![allow(clippy::option_map_unit_fn)]

pub mod block;
pub mod database;
pub mod func;
pub mod error;
pub mod macros;
pub mod transaction;
pub mod contract;

pub use block::IndexedBlock;
pub use database::Settings as DatabaseSettings;
pub use error::Error;
pub use transaction::{IndexedTransaction, TransactionBoilerplate};

use sqlx::PgPool;

/// The result type used through the application code.
type Result<T> = std::result::Result<T, error::Error>;

/// The signature of the ERC721 safeTransactionFrom method
pub const ERC721_SAFE_TRANSFER_FROM_SIGNATURE: &[u8] = &[0x42, 0x84, 0x2e, 0x0e];

/// The signature of the ERC20 approve method
pub const ERC20_APPROVE_SIGNATURE: &[u8] = &[0x09, 0xb6, 0x7f, 0x8e];

/// The signature of the ERC20 transfer method
pub const ERC20_TRANSFER_SIGNATURE: &[u8] = &[0xa9, 0x05, 0x9c, 0xbb];

/// The signature of the ERC20 transferFrom method
pub const ERC20_TRANSFER_FROM_SIGNATURE: &[u8] = &[0x23, 0xb8, 0x72, 0xdd];

#[async_trait::async_trait]
pub trait Insertable: Sized {
    async fn insert(&self, conn: &PgPool) -> Result<()>;
}
