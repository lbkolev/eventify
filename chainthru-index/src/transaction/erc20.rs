pub mod transfer;

pub use transfer::Transfer;

use derive_builder::Builder;
use ethereum_types::H160;

use crate::transaction::DBInsert;

/// The signature of the ERC20 approve method
pub const APPROVE_SIGNATURE: &[u8] = &[0x09, 0xb6, 0x7f, 0x8e];

/// The signature of the ERC20 transfer method
pub const TRANSFER_SIGNATURE: &[u8] = &[0xa9, 0x05, 0x9c, 0xbb];

/// The signature of the ERC20 transferFrom method
pub const TRANSFER_FROM_SIGNATURE: &[u8] = &[0x23, 0xb8, 0x72, 0xdd];

#[derive(Builder, Debug)]
pub struct ERC20 {
    pub contract: H160,
    pub method: Method,
}

/// ERC20 representation
#[derive(Debug, Clone)]
pub enum Method {
    Transfer(Transfer),
}

impl std::fmt::Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Method::Transfer(transfer) => write!(f, "Transfer: {:?}", transfer),
        }
    }
}

impl ERC20 {
    pub fn new(contract: H160, method: Method) -> Self {
        Self { contract, method }
    }

    pub async fn insert(&self, db_conn: &sqlx::PgPool) -> Result<(), sqlx::Error> {
        match &self.method {
            Method::Transfer(transfer) => transfer.insert(self.contract, db_conn).await?,
        }

        Ok(())
    }

    pub async fn insert_where(
        &self,
        db_conn: &sqlx::PgPool,
        where_clause: &str,
    ) -> Result<(), sqlx::Error> {
        match &self.method {
            Method::Transfer(transfer) => {
                transfer
                    .insert_where(self.contract, db_conn, where_clause)
                    .await?
            }
        }

        Ok(())
    }

    pub async fn method_type(&self) -> crate::Result<crate::transaction::TransactionType> {
        todo!()
    }
}
