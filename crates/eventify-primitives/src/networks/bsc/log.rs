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
pub struct BscLog {
    #[serde(flatten)]
    core: CoreLog,
}

impl Insert for BscLog {
    async fn insert(
        &self,
        pool: &sqlx::PgPool,
        _: &Option<alloy_primitives::B256>,
    ) -> Result<(), SqlError> {
        self.core.insert(pool, NetworkKind::Bsc).await
    }
}

impl Emit for BscLog {
    async fn emit(
        &self,
        queue: &redis::Client,
        network: &crate::networks::NetworkKind,
    ) -> eyre::Result<(), EmitError> {
        self.core.emit(queue, network).await
    }
}

impl Log for BscLog {
    fn core(&self) -> &CoreLog {
        &self.core
    }
}
