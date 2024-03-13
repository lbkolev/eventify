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
pub struct PolygonLog {
    #[serde(flatten)]
    core: CoreLog,
}

impl Insert for PolygonLog {
    async fn insert(
        &self,
        pool: &sqlx::PgPool,
        _: &Option<alloy_primitives::B256>,
    ) -> Result<(), SqlError> {
        self.core.insert(pool, NetworkKind::Ethereum).await
    }
}

impl Emit for PolygonLog {
    async fn emit(
        &self,
        queue: &redis::Client,
        network: &crate::networks::NetworkKind,
    ) -> eyre::Result<(), EmitError> {
        self.core.emit(queue, network).await
    }
}

impl Log for PolygonLog {
    fn core(&self) -> &CoreLog {
        &self.core
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_polygon_log() {
        let json = serde_json::json!({
          "address": "0x0000000000000000000000000000000000001010",
          "blockHash": "0xe582012fe151d0ccfbca2619fe9cbd629ddefa26c2ba6db223ec8ecb85a784a0",
          "blockNumber": "0x3409f05",
          "data": "0x0000000000000000000000000000000000000000000000000001c7fabdcb7f1c000000000000000000000000000000000000000000000000107256b94a3591ae00000000000000000000000000000000000000000002d43477d9f9d3b9e9fe8b00000000000000000000000000000000000000000000000010708ebe8c6a129200000000000000000000000000000000000000000002d43477dbc1ce77b57da7",
          "logIndex": "0x197",
          "removed": false,
          "topics": [
            "0x4dfe1bbbcf077ddc3e01291eea2d5c70c2b422b415d95645b9adcfd678cb1d63",
            "0x0000000000000000000000000000000000000000000000000000000000001010",
            "0x00000000000000000000000065dfdda994f3d956630b09f6f26d90c67b7aab36",
            "0x0000000000000000000000007c7379531b2aee82e4ca06d4175d13b9cbeafd49"
          ],
          "transactionHash": "0x3a83d5b3fe3c4a2ddfc4965ba8470adc4dedd58e1864b778dd1c8242a981b436",
          "transactionIndex": "0x7b"
        });

        assert!(serde_json::from_value::<PolygonLog>(json).is_ok());
    }

    #[test]
    fn test_deserialize_empty_polygon_log() {
        let json = serde_json::json!({});

        assert!(serde_json::from_value::<PolygonLog>(json).is_err());
    }
}
