use super::ERC4626;
use crate::{
    networks::{LogKind, ResourceKind},
    traits::{Emit, Insert},
};

use alloy_primitives::B256;
use eyre::Result;
use redis::{Commands, RedisError};
use sqlx::{Error, PgPool};

impl Insert for ERC4626::Deposit {
    async fn insert(
        &self,
        pool: &PgPool,
        schema: &str,
        tx_hash: &Option<B256>,
    ) -> Result<(), Error> {
        let tx = tx_hash.as_ref().map(|v| v.as_slice());
        let sender = self.sender.as_slice();
        let owner = self.owner.as_slice();
        let assets = self.assets.as_le_slice();
        let shares = self.shares.as_le_slice();

        let sql = format!(
            r#"INSERT INTO {schema}.log_deposit (
            tx_hash,
            sender,
            "owner",
            "assets",
            shares )
            VALUES (
                $1, $2, $3, $4, $5
            ) ON CONFLICT DO NOTHING"#,
        );

        sqlx::query(&sql)
            .bind(tx)
            .bind(sender)
            .bind(owner)
            .bind(assets)
            .bind(shares)
            .execute(pool)
            .await?;

        Ok(())
    }
}

impl Emit for ERC4626::Deposit {
    async fn emit<T: serde::Serialize + Send + Sync>(
        &self,
        queue: &redis::Client,
        network: &crate::networks::NetworkKind,
        message: &T,
    ) -> Result<(), RedisError> {
        let mut con = queue.get_connection()?;
        let channel = format!(
            "{}:{}",
            network,
            ResourceKind::Log(LogKind::ERC4626_Deposit)
        );

        con.lpush(channel, serde_json::to_string(message).unwrap())?;
        Ok(())
    }
}

impl Insert for ERC4626::Withdraw {
    async fn insert(
        &self,
        pool: &PgPool,
        schema: &str,
        tx_hash: &Option<B256>,
    ) -> Result<(), Error> {
        let tx = tx_hash.as_ref().map(|v| v.as_slice());
        let sender = self.sender.as_slice();
        let receiver = self.receiver.as_slice();
        let owner = self.owner.as_slice();
        let assets = self.assets.as_le_slice();
        let shares = self.shares.as_le_slice();

        let sql = format!(
            r#"INSERT INTO {schema}.log_withdraw (
            tx_hash,
            sender,
            "receiver",
            "owner",
            "assets",
            shares )
            VALUES (
                $1, $2, $3, $4, $5, $6
            ) ON CONFLICT DO NOTHING"#,
        );

        sqlx::query(&sql)
            .bind(tx)
            .bind(sender)
            .bind(receiver)
            .bind(owner)
            .bind(assets)
            .bind(shares)
            .execute(pool)
            .await?;

        Ok(())
    }
}

impl Emit for ERC4626::Withdraw {
    async fn emit<T: serde::Serialize + Send + Sync>(
        &self,
        queue: &redis::Client,
        network: &crate::networks::NetworkKind,
        message: &T,
    ) -> Result<(), RedisError> {
        let mut con = queue.get_connection()?;
        let channel = format!(
            "{}:{}",
            network,
            ResourceKind::Log(LogKind::ERC4626_Withdraw)
        );

        con.lpush(channel, serde_json::to_string(message).unwrap())?;
        Ok(())
    }
}
