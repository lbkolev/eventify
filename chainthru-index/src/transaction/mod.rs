pub mod erc20;

use async_trait::async_trait;
use ethereum_types::H160;

#[async_trait]
pub trait TransactionInsert {
    async fn insert(&self, contract: H160, db_conn: &sqlx::PgPool) -> Result<(), sqlx::Error>;
}
