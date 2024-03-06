use alloy_primitives::B256;
use eyre::Result;
use redis::Commands;
use sqlx::{Error as SqlError, PgPool};

use super::ERC777;
use crate::{
    networks::{LogKind, ResourceKind},
    traits::{Emit, Insert},
    EmitError,
};

impl Insert for ERC777::Sent {
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
        let amount = self.amount.as_le_slice();
        let data = self.data.as_slice();
        let operator_data = self.operatorData.as_slice();

        let sql = format!(
            r#"INSERT INTO {schema}.log_sent (
            tx_hash,
            operator,
            "from",
            "to",
            amount,
            "data",
            operator_data )
            VALUES (
                $1, $2, $3, $4, $5, $6, $7
            ) ON CONFLICT DO NOTHING"#,
        );

        sqlx::query(&sql)
            .bind(tx)
            .bind(operator)
            .bind(from)
            .bind(to)
            .bind(amount)
            .bind(data)
            .bind(operator_data)
            .execute(pool)
            .await?;

        Ok(())
    }
}

impl Emit for ERC777::Sent {
    async fn emit(
        &self,
        queue: &redis::Client,
        network: &crate::networks::NetworkKind,
    ) -> Result<(), EmitError> {
        let mut con = queue.get_connection()?;

        let channel = format!("{}:{}", network, ResourceKind::Log(LogKind::ERC777_Sent));
        con.lpush(channel, serde_json::to_string(self)?)?;

        Ok(())
    }
}

impl Insert for ERC777::Minted {
    async fn insert(
        &self,
        pool: &PgPool,
        schema: &str,
        tx_hash: &Option<B256>,
    ) -> Result<(), SqlError> {
        let tx = tx_hash.as_ref().map(|v| v.as_slice());
        let operator = self.operator.as_slice();
        let to = self.to.as_slice();
        let amount = self.amount.as_le_slice();
        let data = self.data.as_slice();
        let operator_data = self.operatorData.as_slice();

        let sql = format!(
            r#"INSERT INTO {schema}.log_minted (
            tx_hash,
            operator,
            "to",
            amount,
            "data",
            operator_data )
            VALUES (
                $1, $2, $3, $4, $5, $6
            ) ON CONFLICT DO NOTHING"#,
        );

        sqlx::query(&sql)
            .bind(tx)
            .bind(operator)
            .bind(to)
            .bind(amount)
            .bind(data)
            .bind(operator_data)
            .execute(pool)
            .await?;

        Ok(())
    }
}

impl Emit for ERC777::Minted {
    async fn emit(
        &self,
        queue: &redis::Client,
        network: &crate::networks::NetworkKind,
    ) -> Result<(), EmitError> {
        let mut con = queue.get_connection()?;

        let channel = format!("{}:{}", network, ResourceKind::Log(LogKind::ERC777_Minted));
        con.lpush(channel, serde_json::to_string(self)?)?;

        Ok(())
    }
}

impl Insert for ERC777::Burned {
    async fn insert(
        &self,
        pool: &PgPool,
        schema: &str,
        tx_hash: &Option<B256>,
    ) -> Result<(), SqlError> {
        let tx = tx_hash.as_ref().map(|v| v.as_slice());
        let operator = self.operator.as_slice();
        let from = self.from.as_slice();
        let amount = self.amount.as_le_slice();
        let data = self.data.as_slice();
        let operator_data = self.operatorData.as_slice();

        let sql = format!(
            r#"INSERT INTO {schema}.log_burned (
            tx_hash,
            operator,
            "from",
            amount,
            "data",
            operator_data )
            VALUES (
                $1, $2, $3, $4, $5, $6
            ) ON CONFLICT DO NOTHING"#,
        );

        sqlx::query(&sql)
            .bind(tx)
            .bind(operator)
            .bind(from)
            .bind(amount)
            .bind(data)
            .bind(operator_data)
            .execute(pool)
            .await?;

        Ok(())
    }
}

impl Emit for ERC777::Burned {
    async fn emit(
        &self,
        queue: &redis::Client,
        network: &crate::networks::NetworkKind,
    ) -> Result<(), EmitError> {
        let mut con = queue.get_connection()?;

        let channel = format!("{}:{}", network, ResourceKind::Log(LogKind::ERC777_Burned));
        con.lpush(channel, serde_json::to_string(self)?)?;

        Ok(())
    }
}

impl Insert for ERC777::AuthorizedOperator {
    async fn insert(
        &self,
        pool: &PgPool,
        schema: &str,
        tx_hash: &Option<B256>,
    ) -> Result<(), SqlError> {
        let tx = tx_hash.as_ref().map(|v| v.as_slice());
        let operator = self.operator.as_slice();
        let holder = self.holder.as_slice();

        let sql = format!(
            r#"INSERT INTO {schema}.log_authorized_operator (
            tx_hash,
            operator,
            holder )
            VALUES (
                $1, $2, $3
            ) ON CONFLICT DO NOTHING"#,
        );

        sqlx::query(&sql)
            .bind(tx)
            .bind(operator)
            .bind(holder)
            .execute(pool)
            .await?;

        Ok(())
    }
}

impl Emit for ERC777::AuthorizedOperator {
    async fn emit(
        &self,
        queue: &redis::Client,
        network: &crate::networks::NetworkKind,
    ) -> Result<(), EmitError> {
        let mut con = queue.get_connection()?;

        let channel = format!(
            "{}:{}",
            network,
            ResourceKind::Log(LogKind::ERC777_AuthorizedOperator)
        );
        con.lpush(channel, serde_json::to_string(self)?)?;

        Ok(())
    }
}

impl Insert for ERC777::RevokedOperator {
    async fn insert(
        &self,
        pool: &PgPool,
        schema: &str,
        tx_hash: &Option<B256>,
    ) -> Result<(), SqlError> {
        let tx = tx_hash.as_ref().map(|v| v.as_slice());
        let operator = self.operator.as_slice();
        let holder = self.holder.as_slice();

        let sql = format!(
            r#"INSERT INTO {schema}.log_revoked_operator (
            tx_hash,
            operator,
            holder )
            VALUES (
                $1, $2, $3
            ) ON CONFLICT DO NOTHING"#,
        );

        sqlx::query(&sql)
            .bind(tx)
            .bind(operator)
            .bind(holder)
            .execute(pool)
            .await?;

        Ok(())
    }
}

impl Emit for ERC777::RevokedOperator {
    async fn emit(
        &self,
        queue: &redis::Client,
        network: &crate::networks::NetworkKind,
    ) -> Result<(), EmitError> {
        let mut con = queue.get_connection()?;

        let channel = format!(
            "{}:{}",
            network,
            ResourceKind::Log(LogKind::ERC777_RevokedOperator)
        );
        con.lpush(channel, serde_json::to_string(self)?)?;

        Ok(())
    }
}
