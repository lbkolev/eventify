use alloy_primitives::B256;
use eyre::Result;
use redis::Commands;
use sqlx::{Error as SqlError, PgPool};

use super::ERC1155;
use crate::{
    networks::{LogKind, ResourceKind},
    traits::{Emit, Insert},
    EmitError,
};

impl Insert for ERC1155::TransferSingle {
    async fn insert(
        &self,
        pool: &PgPool,
        schema: &str,
        tx_hash: &Option<B256>,
    ) -> Result<(), SqlError> {
        let tx = tx_hash.as_ref().map(|v| v.as_slice());
        let operator = self.operator.as_slice();
        let from = self.from.as_slice();
        let to = self.to.as_slice();
        let id = self.id.as_le_slice();
        let value = self.value.as_le_slice();

        let sql = format!(
            r#"INSERT INTO {schema}.log_transfer_single (
            tx_hash,
            operator,
            "from",
            "to",
            id,
            value )
            VALUES (
                $1, $2, $3, $4, $5, $6
            ) ON CONFLICT DO NOTHING"#,
        );

        sqlx::query(&sql)
            .bind(tx)
            .bind(operator)
            .bind(from)
            .bind(to)
            .bind(id)
            .bind(value)
            .execute(pool)
            .await?;

        Ok(())
    }
}

impl Emit for ERC1155::TransferSingle {
    async fn emit(
        &self,
        queue: &redis::Client,
        network: &crate::networks::NetworkKind,
    ) -> Result<(), EmitError> {
        let mut con = queue.get_connection()?;

        let channel = format!(
            "{}:{}",
            network,
            ResourceKind::Log(LogKind::ERC1155_TransferSingle)
        );
        con.lpush(channel, serde_json::to_string(self)?)?;

        Ok(())
    }
}

impl Insert for ERC1155::TransferBatch {
    async fn insert(
        &self,
        pool: &PgPool,
        schema: &str,
        tx_hash: &Option<B256>,
    ) -> Result<(), SqlError> {
        let tx = tx_hash.as_ref().map(|v| v.as_slice());
        let operator = self.operator.as_slice();
        let from = self.from.as_slice();
        let to = self.to.as_slice();
        let ids = self.ids.iter().map(|v| v.as_le_slice()).collect::<Vec<_>>();
        let values = self
            .values
            .iter()
            .map(|v| v.as_le_slice())
            .collect::<Vec<_>>();

        let sql = format!(
            r#"INSERT INTO {schema}.log_transfer_batch (
            tx_hash,
            operator,
            "from",
            "to",
            ids,
            values )
            VALUES (
                $1, $2, $3, $4, $5, $6
            ) ON CONFLICT DO NOTHING"#,
        );

        sqlx::query(&sql)
            .bind(tx)
            .bind(operator)
            .bind(from)
            .bind(to)
            .bind(ids)
            .bind(values)
            .execute(pool)
            .await?;

        Ok(())
    }
}

impl Emit for ERC1155::TransferBatch {
    async fn emit(
        &self,
        queue: &redis::Client,
        network: &crate::networks::NetworkKind,
    ) -> Result<(), EmitError> {
        let mut con = queue.get_connection()?;

        let channel = format!(
            "{}:{}",
            network,
            ResourceKind::Log(LogKind::ERC1155_TransferBatch)
        );
        con.lpush(channel, serde_json::to_string(self)?)?;

        Ok(())
    }
}

impl Insert for ERC1155::URI {
    async fn insert(
        &self,
        pool: &PgPool,
        schema: &str,
        tx_hash: &Option<B256>,
    ) -> Result<(), SqlError> {
        let tx = tx_hash.as_ref().map(|v| v.as_slice());
        let value = self.value.as_str();
        let id = self.id.as_le_slice();

        let sql = format!(
            r#"INSERT INTO {schema}.log_uri (
            tx_hash,
            "value",
            id )
            VALUES (
                $1, $2, $3
            ) ON CONFLICT DO NOTHING"#,
        );

        sqlx::query(&sql)
            .bind(tx)
            .bind(value)
            .bind(id)
            .execute(pool)
            .await?;

        Ok(())
    }
}

impl Emit for ERC1155::URI {
    async fn emit(
        &self,
        queue: &redis::Client,
        network: &crate::networks::NetworkKind,
    ) -> Result<(), EmitError> {
        let mut con = queue.get_connection()?;

        let channel = format!("{}:{}", network, ResourceKind::Log(LogKind::ERC1155_URI));
        con.lpush(channel, serde_json::to_string(self)?)?;

        Ok(())
    }
}
