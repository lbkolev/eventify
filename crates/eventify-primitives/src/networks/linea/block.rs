use std::{fmt::Debug, hash::Hash};

use alloy_primitives::U256;
use eyre::Result;
use redis::AsyncCommands;
use sqlx::FromRow;
use utoipa::ToSchema;

use crate::{
    networks::{core::CoreBlock, NetworkKind, ResourceKind},
    traits::{Block, Emit, Insert},
    EmitError,
};

#[derive(
    Clone,
    Debug,
    Default,
    serde::Deserialize,
    serde::Serialize,
    PartialEq,
    Eq,
    Hash,
    FromRow,
    ToSchema,
)]
pub struct LineaBlock {
    #[serde(flatten)]
    core: CoreBlock,

    #[serde(rename = "baseFeePerGas")]
    pub base_fee: Option<U256>,
    #[serde(rename = "totalDifficulty")]
    pub total_difficulty: Option<U256>,
}

impl Block for LineaBlock {
    fn core(&self) -> &CoreBlock {
        &self.core
    }
}

impl Insert for LineaBlock {
    async fn insert(
        &self,
        pool: &sqlx::PgPool,
        _: &Option<alloy_primitives::B256>,
    ) -> Result<(), sqlx::Error> {
        self.core.insert(pool, NetworkKind::Linea).await
    }
}

impl Emit for LineaBlock {
    async fn emit(&self, queue: &redis::Client, network: &NetworkKind) -> Result<(), EmitError> {
        let mut con = queue.get_async_connection().await?;

        let channel = format!("{}:{}", network, ResourceKind::Block);
        con.lpush(channel, serde_json::to_string(self)?).await?;

        Ok(())
    }
}
