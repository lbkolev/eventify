use alloy_primitives::B256;

use crate::utils::{setup_test_db, setup_test_redis, teardown_test_db};

use eventify_primitives::{
    networks::{
        arbitrum::{ArbitrumBlock, ArbitrumLog},
        NetworkKind,
    },
    EmitT, InsertT, LogT,
};

#[tokio::test]
async fn test_insert_and_emit_arbitrum_block() {
    let (pool, db_name) = setup_test_db().await.unwrap();
    let redis = setup_test_redis().await;

    let json = serde_json::json!(
        {
            "baseFeePerGas": "0x5f5e100",
            "difficulty": "0x1",
            "extraData": "0x806ecf336ef1bd79c9314ebb2c2b0360b7946bb6dd9292f0d5d1da1132fba3ac",
            "gasLimit": "0x4000000000000",
            "gasUsed": "0x4272c1",
            "hash": "0xcf8277f690178d87e8dafbb2e08baf8ec0cb38429bf661bedbb6ac70742e1173",
            "logsBloom": "0x0000008000000000000400000000000000000000000000000000000001100010000000000000000000000000000000000000400108002000004000000020000000000101000000080000000800012000000000000000000200000000800000000000000800000000000000140000000000002000080040000000001000080000000800000000880000000000000000000000000000000400000000000000004002000000a000000000000020000000800000000040000008000400000080000000000002000000000000000000000000004000000000200000000000000400000010000000000000000800000000000000000000000000000000000000000000",
            "miner": "0xa4b000000000000000000073657175656e636572",
            "mixHash": "0x000000000001b3610000000001286a87000000000000000b0000000000000000",
            "nonce": "0x0000000000160c06",
            "number": "0xb524434",
            "parentHash": "0x06216d98e6b7a0b5d4991db42c32b724fa5aae9d812970b20d27e042e1f8a72c",
            "receiptsRoot": "0x03ae14dd416662aa2551c1204cc506564e07522c44d5a23c9426e8c11ebcadfa",
            "sha3Uncles": "0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347",
            "stateRoot": "0x67d5a71cd1d0553ae3304a4d8a4b1916dea96c8450e95e8b126f1964db1b6db1",
            "timestamp": "0x65f19155",
            "totalDifficulty": "0x9ff66ec",
            "transactionsRoot": "0xc5742826a25bcf43fa5f4706e30422af2abc790ab199dd0338fb09ed06946bfa"
        }
    );

    let block = serde_json::from_value::<ArbitrumBlock>(json).unwrap();
    block.insert(&pool, &None).await.unwrap();
    block.emit(&redis, &NetworkKind::Arbitrum).await.unwrap();

    teardown_test_db(pool, &db_name).await.unwrap();
}

#[tokio::test]
async fn test_insert_and_emit_arbitrum_log() {
    let (pool, db_name) = setup_test_db().await.unwrap();
    let redis = setup_test_redis().await;

    let json = serde_json::json!(
        {
            "address": "0xa6c895eb332e91c5b3d00b7baeeaae478cc502da",
            "blockHash": "0xf66779d342803ee3f1ed4cf44047b3f4d17e20b7a856f8a9ac5dacf2d52d4437",
            "blockNumber": "0xb52448e",
            "data": "0x00000000000000000000000000000000000000000000000000108d3bfda316e7",
            "logIndex": "0x0",
            "removed": false,
            "topics": [
              "0x8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925",
              "0x000000000000000000000000de771b321326954de7af0657c1f8008cb3f5937d",
              "0x00000000000000000000000000000000005bbb0ef59571e58418f9a4357b68a0"
            ],
            "transactionHash": "0xf4016512ac989d6af2d9d70adbac5e1b596dab1e03364db32488e0643d5218e4",
            "transactionIndex": "0x1"
        }
    );

    let log = serde_json::from_value::<ArbitrumLog>(json).unwrap();
    log.insert(&pool, &log.core().tx_hash).await.unwrap();
    log.emit(&redis, &NetworkKind::Arbitrum).await.unwrap();

    teardown_test_db(pool, &db_name).await.unwrap();
}
