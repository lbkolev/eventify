pub mod transfer;

use ethereum_types::H160;

use transfer::Transfer;

pub const TRANSFER_SIGNATURE: &[u8] = &[0xa9, 0x05, 0x9c, 0xbb];

pub struct ERC20 {
    pub contract: H160,
    pub method: Method,
}

/// ERC20 representation
#[derive(Debug)]
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
}
