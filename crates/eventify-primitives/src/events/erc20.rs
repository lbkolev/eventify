use super::ERC20;
use crate::{
    networks::{LogKind, ResourceKind},
    traits::{Emit, Insert},
};

use alloy_primitives::B256;
use eyre::Result;
use redis::{Commands, RedisError};
use sqlx::{Error, PgPool};

impl Insert for ERC20::Transfer {
    async fn insert(
        &self,
        pool: &PgPool,
        schema: &str,
        tx_hash: &Option<B256>,
    ) -> Result<(), Error> {
        let tx = tx_hash.as_ref().map(|v| v.as_slice());
        let from = self.from.as_slice();
        let to = self.to.as_slice();
        let value = self.value.as_le_slice();

        let sql = format!(
            r#"INSERT INTO {schema}.log_transfer (
            tx_hash,
            "from",
            "to",
            value )
            VALUES (
                $1, $2, $3, $4
            ) ON CONFLICT DO NOTHING"#,
        );

        sqlx::query(&sql)
            .bind(tx)
            .bind(from)
            .bind(to)
            .bind(value)
            .execute(pool)
            .await?;

        Ok(())
    }
}

impl Emit for ERC20::Transfer {
    async fn emit<T: serde::Serialize + Send + Sync>(
        &self,
        queue: &redis::Client,
        network: &crate::networks::NetworkKind,
        message: &T,
    ) -> Result<(), RedisError> {
        let mut con = queue.get_connection()?;
        let channel = format!("{}:{}", network, ResourceKind::Log(LogKind::ERC20_Transfer));

        con.lpush(channel, serde_json::to_string(message).unwrap())?;
        Ok(())
    }
}

impl Insert for ERC20::Approval {
    async fn insert(
        &self,
        pool: &PgPool,
        schema: &str,
        tx_hash: &Option<alloy_primitives::B256>,
    ) -> Result<(), Error> {
        let tx = tx_hash.as_ref().map(|v| v.as_slice());
        let owner = self.owner.as_slice();
        let spender = self.spender.as_slice();
        let value = self.value.as_le_slice();

        let sql = format!(
            r#"INSERT INTO {schema}.log_approval (
            tx_hash,
            "owner",
            spender,
            "value" )
            VALUES (
                $1, $2, $3, $4
            ) ON CONFLICT DO NOTHING"#,
        );

        sqlx::query(&sql)
            .bind(tx)
            .bind(owner)
            .bind(spender)
            .bind(value)
            .execute(pool)
            .await?;

        Ok(())
    }
}

impl Emit for ERC20::Approval {
    async fn emit<T: serde::Serialize + Send + Sync>(
        &self,
        queue: &redis::Client,
        network: &crate::networks::NetworkKind,
        message: &T,
    ) -> Result<(), RedisError> {
        let mut con = queue.get_connection()?;
        let channel = format!("{}:{}", network, ResourceKind::Log(LogKind::ERC20_Approval));

        con.lpush(channel, serde_json::to_string(message).unwrap())?;
        Ok(())
    }
}
