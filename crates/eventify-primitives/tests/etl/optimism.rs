use alloy_primitives::B256;

use crate::utils::{setup_test_db, setup_test_redis, teardown_test_db};

use eventify_primitives::{
    networks::{
        optimism::{OptimismBlock, OptimismLog},
        NetworkKind,
    },
    EmitT, InsertT, LogT,
};

#[tokio::test]
async fn test_insert_and_emit_optimism_block() {
    let (pool, db_name) = setup_test_db().await.unwrap();
    let redis = setup_test_redis().await;

    let json = serde_json::json!(
        {
            "baseFeePerGas": "0x102",
            "difficulty": "0x0",
            "extraData": "0x",
            "gasLimit": "0x1c9c380",
            "gasUsed": "0x2b4f63",
            "hash": "0xb7c40edeafee6b701ef7e2bed66637d055631b7fcf60d7edd1b6d444a67ebed1",
            "logsBloom": "0x02000800400000000000008000408000000004000000000001040200000000000000012000000040100000100000080000000040200020102000000000240041000004000000000822020008008000001480000040000000000000008001400110040000000000200080000000000002000000022000008000040010810802000102101000000000410000001600000001200081802400010010005000001004022000100000040000000008000028001004000000480000000000010001000540000003080000000000000200100000208000000000000000002000004400000010000000810000000000041100000000400001008020400000000000030000",
            "miner": "0x4200000000000000000000000000000000000011",
            "mixHash": "0x730b169d9e0b3a1d9d615e6987d32e1f10e70575b7df72306148bcabd5c2f6b7",
            "nonce": "0x0000000000000000",
            "number": "0x6feda74",
            "parentHash": "0x166b8c60e7b6f92723c30bcbf0a9f1f564edd55a806d1b1b3260657ed6d25429",
            "receiptsRoot": "0xc84cda83faeb987537b4145645d2572c128ec6b3091ad3877215420c8f636320",
            "sha3Uncles": "0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347",
            "stateRoot": "0x4c26af22d402a143ca5cb0e9ebd922650025395ec8f85d392194935a7bf678ab",
            "timestamp": "0x65f18ea1",
            "totalDifficulty": "0x0",
            "transactionsRoot": "0x3de4789f9cc563d72f3aa9ebb5043febc12937c57977e0a78c53f294ebb80191",
            "withdrawalsRoot": "0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421"
          }
    );

    let block = serde_json::from_value::<OptimismBlock>(json).unwrap();
    block.insert(&pool, &None).await.unwrap();
    block.emit(&redis, &NetworkKind::Optimism).await.unwrap();

    teardown_test_db(pool, &db_name).await.unwrap();
}

#[tokio::test]
async fn test_insert_and_emit_optimism_log() {
    let (pool, db_name) = setup_test_db().await.unwrap();
    let redis = setup_test_redis().await;

    let json = serde_json::json!(
        {
            "address": "0xdc6ff44d5d932cbd77b52e5612ba0529dc6226f1",
            "blockHash": "0x632d4a7b6c082ac7ccca840ca7991ae529c20f5e069082fdecf75bb6db80e4df",
            "blockNumber": "0x6feda85",
            "data": "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
            "logIndex": "0x4",
            "removed": false,
            "topics": [
              "0x8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925",
              "0x0000000000000000000000005b14de8def9dbe1f4c52d98471dddd10f28029b8",
              "0x000000000000000000000000e9c8c97ee34dca7d80b76cc9906106cc50059d2d"
            ],
            "transactionHash": "0x2c6aeb6c2151fa6950e13a06ff967ebe31cb5034564721ba2ba3bfec46c9650d",
            "transactionIndex": "0x5"
          }
    );

    let log = serde_json::from_value::<OptimismLog>(json).unwrap();
    log.insert(&pool, &log.core().tx_hash).await.unwrap();
    log.emit(&redis, &NetworkKind::Optimism).await.unwrap();

    teardown_test_db(pool, &db_name).await.unwrap();
}
