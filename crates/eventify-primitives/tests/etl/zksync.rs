use alloy_primitives::B256;

use crate::utils::{setup_test_db, setup_test_redis, teardown_test_db};

use eventify_primitives::{
    networks::{
        zksync::{ZksyncBlock, ZksyncLog},
        NetworkKind,
    },
    EmitT, InsertT, LogT,
};

#[tokio::test]
async fn test_insert_and_emit_zksync_block() {
    let (pool, db_name) = setup_test_db().await.unwrap();
    let redis = setup_test_redis().await;

    let json = serde_json::json!({
      "hash": "0x8af7b57f6c931525581667c4a8535e74e0d2cb546365a9c97a460f31a89ab4ab",
      "parentHash": "0x0000000000000000000000000000000000000000000000000000000000000000",
      "sha3Uncles": "0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347",
      "miner": "0x0000000000000000000000000000000000000000",
      "stateRoot": "0x0000000000000000000000000000000000000000000000000000000000000000",
      "transactionsRoot": "0x0000000000000000000000000000000000000000000000000000000000000000",
      "receiptsRoot": "0x0000000000000000000000000000000000000000000000000000000000000000",
      "number": "0x114496",
      "gasUsed": "0x0",
      "gasLimit": "0x0",
      "extraData": "0x",
      "logsBloom": "0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
      "timestamp": "0x65f004ab",
      "difficulty": "0x0",
      "mixHash": null,
      "nonce": null
    });

    let block = serde_json::from_value::<ZksyncBlock>(json).unwrap();
    block.insert(&pool, &None).await.unwrap();
    block.emit(&redis, &NetworkKind::Zksync).await.unwrap();

    teardown_test_db(pool, &db_name).await.unwrap();
}

#[tokio::test]
async fn test_insert_and_emit_zksync_log() {
    let (pool, db_name) = setup_test_db().await.unwrap();
    let redis = setup_test_redis().await;

    let json = serde_json::json!({
      "address": "0x000000000000000000000000000000000000800a",
      "topics": [
        "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef",
        "0x0000000000000000000000000000000000000000000000000000000000008001",
        "0x0000000000000000000000008b1d48a69acebc6eb201e2f4d162a002203bfe8e"
      ],
      "data": "0x00000000000000000000000000000000000000000000000000014b2fff91c500",
      "blockHash": "0x82584884d96e933d9c585e7ede5cc394b6d646bf1c84598032178cb5b9837eea",
      "blockNumber": "0x1145db",
      "l1BatchNumber": null,
      "transactionHash": "0x9c4e447e4fe935743739bb643556b9cca5dc1e76a394fb4bb4e95ce07f2e494b",
      "transactionIndex": "0x0",
      "logIndex": "0x5",
      "transactionLogIndex": "0x5",
      "logType": null,
      "removed": false
    });

    let log = serde_json::from_value::<ZksyncLog>(json).unwrap();
    log.insert(&pool, &None).await.unwrap();
    log.emit(&redis, &NetworkKind::Zksync).await.unwrap();

    teardown_test_db(pool, &db_name).await.unwrap();
}
