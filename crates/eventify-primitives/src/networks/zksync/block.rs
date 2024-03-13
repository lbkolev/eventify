use std::{fmt::Debug, hash::Hash};

use alloy_primitives::B256;
use eyre::Result;

use sqlx::{Error as SqlError, FromRow};
use utoipa::ToSchema;

use crate::{
    networks::{core::CoreBlock, NetworkKind},
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
pub struct ZksyncBlock {
    #[serde(flatten)]
    core: CoreBlock,
}

impl Block for ZksyncBlock {
    fn core(&self) -> &CoreBlock {
        &self.core
    }
}

impl Insert for ZksyncBlock {
    async fn insert(&self, pool: &sqlx::PgPool, _: &Option<B256>) -> Result<(), SqlError> {
        self.core().insert(pool, NetworkKind::Zksync).await
    }
}

impl Emit for ZksyncBlock {
    // TODO: do we really need the network here?
    async fn emit(&self, queue: &redis::Client, network: &NetworkKind) -> Result<(), EmitError> {
        self.core().emit(queue, network).await
    }
}

#[cfg(test)]
mod tests {
    use super::ZksyncBlock;

    #[test]
    fn deserialize_zksync_block() {
        let json = serde_json::json!(
            {
                "hash": "0x73c37ce80f617edf559dbc824d968de772e9a894c6faea4e0e1c0fe7267ed1f5",
                "parentHash": "0x0000000000000000000000000000000000000000000000000000000000000000",
                "sha3Uncles": "0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347",
                "miner": "0x0000000000000000000000000000000000000000",
                "stateRoot": "0x0000000000000000000000000000000000000000000000000000000000000000",
                "transactionsRoot": "0x0000000000000000000000000000000000000000000000000000000000000000",
                "receiptsRoot": "0x0000000000000000000000000000000000000000000000000000000000000000",
                "number": "0x1b82257",
                "gasUsed": "0x0",
                "gasLimit": "0x0",
                "extraData": "0x",
                "logsBloom": "0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
                "timestamp": "0x65f1682a",
                "difficulty": "0x0",
                "mixHash": null,
                "nonce": null
              }
        );

        assert!(serde_json::from_value::<ZksyncBlock>(json).is_ok());
    }

    #[test]
    fn deserialize_empty_zksync_block() {
        let json = serde_json::json!({});

        assert!(serde_json::from_value::<ZksyncBlock>(json).is_err());
    }
}
