use alloy_primitives::B256;

use crate::utils::{setup_test_db, setup_test_redis, teardown_test_db};

use eventify_primitives::{
    networks::{
        linea::{LineaBlock, LineaLog},
        NetworkKind,
    },
    EmitT, InsertT, LogT,
};

#[tokio::test]
async fn test_insert_and_emit_linea_block() {
    let (pool, db_name) = setup_test_db().await.unwrap();
    let redis = setup_test_redis().await;

    let json = serde_json::json!(
        {
            "baseFeePerGas": "0x7",
            "difficulty": "0x2",
            "extraData": "0x00000000000000000000000000000000000000000000000000000000000000009caed407b4813482ba0c9f96773b3929983906882332826050d0e533db8580706e2e74fe2648ab12540a698990be88ca4b490bb15368a74a29f96cf90aa87cab01",
            "gasLimit": "0x3a2c940",
            "gasUsed": "0x4a40c2",
            "hash": "0xfe90eba5f1e71e2a68e65a34eafa8e5a3215e6d3b0c045d22937b44b7ff93fcd",
            "logsBloom": "0x42020a242004690440800008021000a088882801011805148084000002464e025c00a0280022001004000024d004000040103ec0089000260644413002a4080092802120010420400109609a0600b21140400200404c004020110054826c0004c68870804284001002008427018d3d81840000023d10800a110480149808b401400802608010840b081010040004020000111001a00220cb0c911000040120400280000288102000000140008008801020005000400040290024100023000d8102408a02100080000010182442201880020e04808045220b04109000300072080010640e22208104208800840000200040000161300080e302004130821804a0",
            "miner": "0x0000000000000000000000000000000000000000",
            "mixHash": "0x0000000000000000000000000000000000000000000000000000000000000000",
            "nonce": "0x0000000000000000",
            "number": "0x2bc39c",
            "parentHash": "0x81abd96feabb9167a8fe3ddd43e0ef14d0f97e54c17135c5a8f4a4f1d0a9ee47",
            "receiptsRoot": "0x6205fe1e072d479495e2d813aedbd90c8521a77b729bf0bbdc2ad40071cb9257",
            "sha3Uncles": "0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347",
            "stateRoot": "0x3ffaafbaa15d7f6c1001713e1150e8718c7b38601c207ffa0c0e059434158c62",
            "timestamp": "0x65f19232",
            "totalDifficulty": "0x578739",
            "transactionsRoot": "0x55efe664c9f2fe90d44e8f0659f97431d8b2147124287fa689e24bcf151b03ef"
        }
    );

    let block = serde_json::from_value::<LineaBlock>(json).unwrap();
    block.insert(&pool, &None).await.unwrap();
    block.emit(&redis, &NetworkKind::Linea).await.unwrap();

    teardown_test_db(pool, &db_name).await.unwrap();
}

#[tokio::test]
async fn test_insert_and_emit_linea_log() {
    let (pool, db_name) = setup_test_db().await.unwrap();
    let redis = setup_test_redis().await;

    let json = serde_json::json!(
        {
            "address": "0xd1a3abf42f9e66be86cfdea8c5c2c74f041c5e14",
            "blockHash": "0x469b6db282d24a0fd3dd0075cd3c0fe8935b1fd6474e2d01033d236e70bd87f5",
            "blockNumber": "0x2bc3a3",
            "data": "0x",
            "logIndex": "0x24",
            "removed": false,
            "topics": [
              "0x5c0bf6ba470f83fe17f0b8fd9fdf4799eaeb1b63bbf39e4868fc7e0798e7abeb",
              "0x000000000000000000000000f1b61071de7d67fb4c1d865ed8d488ccbf4a5eb9",
              "0x4db487c8caeb22ba575eb0ea660249f76e764c35ec5cdd9d8dedf517c0c1a14f",
              "0xa2cef2f150a9165808d14994150be8f18c1d307cf4b262b80af7874368d48002"
            ],
            "transactionHash": "0xdc44fd59114e47425404d8961ab2dcc926234ff321110a7c9261a2c01158be8b",
            "transactionIndex": "0x14"
        }
    );

    let log = serde_json::from_value::<LineaLog>(json).unwrap();
    log.insert(&pool, &log.core().tx_hash).await.unwrap();
    log.emit(&redis, &NetworkKind::Linea).await.unwrap();

    teardown_test_db(pool, &db_name).await.unwrap();
}
