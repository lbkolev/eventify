pub mod erc20;

use async_trait::async_trait;
use ethereum_types::H160;

#[derive(Debug)]
pub enum TransactionType {
    ERC20,
    ERC721,
    ERC1155,
    Other,
}

#[async_trait]
pub trait DBInsert {
    async fn insert(&self, contract: H160, db_conn: &sqlx::PgPool) -> Result<(), sqlx::Error>;

    async fn insert_where(
        &self,
        contract: H160,
        db_conn: &sqlx::PgPool,
        where_clause: &str,
    ) -> Result<(), sqlx::Error>;
}
