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
pub struct BaseLog {
    #[serde(flatten)]
    core: CoreLog,
}

impl Insert for BaseLog {
    async fn insert(
        &self,
        pool: &sqlx::PgPool,
        _: &Option<alloy_primitives::B256>,
    ) -> Result<(), SqlError> {
        self.core.insert(pool, NetworkKind::Base).await
    }
}

impl Emit for BaseLog {
    async fn emit(
        &self,
        queue: &redis::Client,
        network: &crate::networks::NetworkKind,
    ) -> eyre::Result<(), EmitError> {
        self.core.emit(queue, network).await
    }
}

impl Log for BaseLog {
    fn core(&self) -> &CoreLog {
        &self.core
    }
}
