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
pub struct PolygonBlock {
    #[serde(flatten)]
    core: CoreBlock,

    #[serde(rename = "baseFeePerGas")]
    pub base_fee: Option<U256>,
    #[serde(rename = "totalDifficulty")]
    pub total_difficulty: Option<U256>,
}

impl Block for PolygonBlock {
    fn core(&self) -> &CoreBlock {
        &self.core
    }
}

impl Insert for PolygonBlock {
    async fn insert(
        &self,
        pool: &sqlx::PgPool,
        _: &Option<alloy_primitives::B256>,
    ) -> Result<(), sqlx::Error> {
        self.core.insert(pool, NetworkKind::Polygon).await
    }
}

impl Emit for PolygonBlock {
    async fn emit(&self, queue: &redis::Client, network: &NetworkKind) -> Result<(), EmitError> {
        let mut con = queue.get_async_connection().await?;

        let channel = format!("{}:{}", network, ResourceKind::Block);
        con.lpush(channel, serde_json::to_string(self)?).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_polygon_block() {
        let json = serde_json::json!(
            {
                "parentHash": "0x4912fcaf7296fb5bdb46d48e2c0b3cc41cb3aa1850e2a1fbeae6932b455c8d0d",
                "sha3Uncles": "0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347",
                "miner": "0x0000000000000000000000000000000000000000",
                "stateRoot": "0xc935b71945c996e1b01aef1561c07687c4fcec10f93a150da9e33e40a768ffe4",
                "transactionsRoot": "0x116d6f925a8b3abe7c390ccc7f0fb10b40c4d15d3c28ff3df9816c6c3f91ab4a",
                "receiptsRoot": "0x4e82db0e69291538acaedc49446154b4fbdf01cf14d97855e5b826f5fd431422",
                "logsBloom": "0x176f24aae180d2c873b520bebc350e067ac8e17c884185a6bdb6c8ff90a0f9b15210158c6ec791fb461a1b3057157e914845c7ad6b47fc02261a0d9094e62b08295cc9187efb442c56f01c19f1e75affe28194e6bdc6d5b41f47d422823b6e5911771bf75a4a8414537e0c09b7c68ac8534632a32bda762df531d036e04916ebe7fba905a5ac71bc5121ffc53c9380e3d71b37ddab265ece6b5fad4760a40a9d7fd0737655c02e2135a1efbf78f892056fd94c7594d78c87cd889fb21a6a29c9ab8477f3841b6d97e2abeb280f9875dce256e20b539bf41a5f3feb53eacba2c512f58dc829acce8a7b853ddeb082dd6a5fb8c78b91e12a631065d93326b6c99a",
                "difficulty": "0x16",
                "number": "0x3412ab0",
                "gasLimit": "0x1c9c380",
                "gasUsed": "0xfcf8cb",
                "timestamp": "0x65f1686f",
                "extraData": "0xd88301020783626f7289676f312e32302e3134856c696e757800000000000000817c89d9bd2e4714e027a526d7ba90692ffe08dc71fa0831f8f6412f53fa747704a6c45259b8744e88458ea2c839a6c902f944059c0fc0b5610bfea22210874701",
                "mixHash": "0x0000000000000000000000000000000000000000000000000000000000000000",
                "nonce": "0x0000000000000000",
                "baseFeePerGas": "0x112fe5ea31",
                "withdrawalsRoot": null,
                "blobGasUsed": null,
                "excessBlobGas": null,
                "parentBeaconBlockRoot": null,
                "hash": "0xf70473275b652c4542cc90e3d6e807679522fc7f8a998d7a4dba6f5420f30463"
              }
        );

        assert!(serde_json::from_value::<PolygonBlock>(json.clone()).is_ok());
    }

    #[test]
    fn deserialize_empty_polygon_block() {
        let json = serde_json::json!({});

        assert!(serde_json::from_value::<PolygonBlock>(json).is_err());
    }
}
