use alloy_primitives::B256;
use eyre::Result;
use redis::Commands;
use sqlx::{Error as SqlError, PgPool};

use super::ERC721;
use crate::{
    networks::{LogKind, ResourceKind},
    traits::{Emit, Insert},
    EmitError,
};

impl Insert for ERC721::Transfer {
    async fn insert(
        &self,
        pool: &PgPool,
        schema: &str,
        tx_hash: &Option<B256>,
    ) -> Result<(), SqlError> {
        let tx = tx_hash.as_ref().map(|v| v.as_slice());
        let from = self.from.as_slice();
        let to = self.to.as_slice();
        let token_id = self.tokenId.as_le_slice();

        let sql = format!(
            r#"INSERT INTO {schema}.log_erc721_transfer (
            tx_hash,
            "from",
            "to",
            token_id )
            VALUES (
                $1, $2, $3, $4
            ) ON CONFLICT DO NOTHING"#,
        );

        sqlx::query(&sql)
            .bind(tx)
            .bind(from)
            .bind(to)
            .bind(token_id)
            .execute(pool)
            .await?;

        Ok(())
    }
}

impl Emit for ERC721::Transfer {
    async fn emit(
        &self,
        queue: &redis::Client,
        network: &crate::networks::NetworkKind,
    ) -> Result<(), EmitError> {
        let mut con = queue.get_connection()?;

        let channel = format!(
            "{}:{}",
            network,
            ResourceKind::Log(LogKind::ERC721_Transfer)
        );
        con.lpush(channel, serde_json::to_string(self)?)?;

        Ok(())
    }
}

impl Insert for ERC721::Approval {
    async fn insert(
        &self,
        pool: &PgPool,
        schema: &str,
        tx_hash: &Option<B256>,
    ) -> Result<(), SqlError> {
        let tx = tx_hash.as_ref().map(|v| v.as_slice());
        let owner = self.owner.as_slice();
        let approved = self.approved.as_slice();
        let token_id = self.tokenId.as_le_slice();

        let sql = format!(
            r#"INSERT INTO {schema}.log_erc721_approval (
            tx_hash,
            "owner",
            approved,
            token_id )
            VALUES (
                $1, $2, $3, $4
            ) ON CONFLICT DO NOTHING"#,
        );

        sqlx::query(&sql)
            .bind(tx)
            .bind(owner)
            .bind(approved)
            .bind(token_id)
            .execute(pool)
            .await?;

        Ok(())
    }
}

impl Emit for ERC721::Approval {
    async fn emit(
        &self,
        queue: &redis::Client,
        network: &crate::networks::NetworkKind,
    ) -> Result<(), EmitError> {
        let mut con = queue.get_connection()?;

        let channel = format!(
            "{}:{}",
            network,
            ResourceKind::Log(LogKind::ERC721_Approval)
        );
        con.lpush(channel, serde_json::to_string(self)?)?;

        Ok(())
    }
}

impl Insert for ERC721::ApprovalForAll {
    async fn insert(
        &self,
        pool: &PgPool,
        schema: &str,
        tx_hash: &Option<B256>,
    ) -> Result<(), SqlError> {
        let tx = tx_hash.as_ref().map(|v| v.as_slice());
        let owner = self.owner.as_slice();
        let operator = self.operator.as_slice();
        let approved = self.approved;

        let sql = format!(
            r#"INSERT INTO {schema}.log_approval_for_all (
            tx_hash,
            "owner",
            operator,
            approved )
            VALUES (
                $1, $2, $3, $4
            ) ON CONFLICT DO NOTHING"#,
        );

        sqlx::query(&sql)
            .bind(tx)
            .bind(owner)
            .bind(operator)
            .bind(approved)
            .execute(pool)
            .await?;

        Ok(())
    }
}

impl Emit for ERC721::ApprovalForAll {
    async fn emit(
        &self,
        queue: &redis::Client,
        network: &crate::networks::NetworkKind,
    ) -> Result<(), EmitError> {
        let mut con = queue.get_connection()?;

        let channel = format!(
            "{}:{}",
            network,
            ResourceKind::Log(LogKind::ERC721_ApprovalForAll)
        );
        con.lpush(channel, serde_json::to_string(self)?)?;

        Ok(())
    }
}
