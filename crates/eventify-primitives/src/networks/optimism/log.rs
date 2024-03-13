use eyre::Result;

use sqlx::{prelude::FromRow, Error as SqlError};
use utoipa::ToSchema;

use crate::{
    networks::{core::CoreLog, NetworkKind},
    traits::{Emit, Insert, Log},
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
pub struct OptimismLog {
    #[serde(flatten)]
    core: CoreLog,
}

impl Insert for OptimismLog {
    async fn insert(
        &self,
        pool: &sqlx::PgPool,
        _: &Option<alloy_primitives::B256>,
    ) -> Result<(), SqlError> {
        self.core.insert(pool, NetworkKind::Optimism).await
    }
}

impl Emit for OptimismLog {
    async fn emit(
        &self,
        queue: &redis::Client,
        network: &crate::networks::NetworkKind,
    ) -> eyre::Result<(), EmitError> {
        self.core.emit(queue, network).await
    }
}

impl Log for OptimismLog {
    fn core(&self) -> &CoreLog {
        &self.core
    }
}
