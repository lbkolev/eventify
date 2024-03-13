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
pub struct AvalancheBlock {
    #[serde(flatten)]
    core: CoreBlock,

    #[serde(rename = "baseFeePerGas")]
    pub base_fee: Option<U256>,
    #[serde(rename = "totalDifficulty")]
    pub total_difficulty: Option<U256>,
}

impl Block for AvalancheBlock {
    fn core(&self) -> &CoreBlock {
        &self.core
    }
}

impl Insert for AvalancheBlock {
    async fn insert(
        &self,
        pool: &sqlx::PgPool,
        _: &Option<alloy_primitives::B256>,
    ) -> Result<(), sqlx::Error> {
        self.core.insert(pool, NetworkKind::Avalanche).await
    }
}

impl Emit for AvalancheBlock {
    async fn emit(&self, queue: &redis::Client, network: &NetworkKind) -> Result<(), EmitError> {
        let mut con = queue.get_async_connection().await?;

        let channel = format!("{}:{}", network, ResourceKind::Block);
        con.lpush(channel, serde_json::to_string(self)?).await?;

        Ok(())
    }
}
