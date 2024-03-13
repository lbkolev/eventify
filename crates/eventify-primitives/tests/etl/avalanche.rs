use alloy_primitives::B256;

use crate::utils::{setup_test_db, setup_test_redis, teardown_test_db};

use eventify_primitives::{
    networks::{
        avalanche::{AvalancheBlock, AvalancheLog},
        NetworkKind,
    },
    EmitT, InsertT, LogT,
};

#[tokio::test]
async fn test_insert_and_emit_avalanche_block() {
    let (pool, db_name) = setup_test_db().await.unwrap();
    let redis = setup_test_redis().await;

    let json = serde_json::json!(
        {
            "baseFeePerGas": "0x5d21dba00",
            "blockGasCost": "0x0",
            "difficulty": "0x1",
            "extDataGasUsed": "0x0",
            "extDataHash": "0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421",
            "extraData": "0x0000000000262e330000000000090658000000000025eec1000000000000000000000000003345ed00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
            "gasLimit": "0xe4e1c0",
            "gasUsed": "0x1fd2c9",
            "hash": "0x70c53b82d58e8c70f77c3dc304c194446dc2e739f9e1bf8e11d7d6eba4399e10",
            "logsBloom": "0x112a000010400808008020008000000000800000000000002200200000100040400000000404004000500000001400000001000200000e8000050800002020000810c000bc0040028080440800000020004004480241000420000004802004008010001082002a00044200000000090000000400001a060100000010040010001002000480000002000000000000240000020111080000090000006040000c00020000804080000041000004000000020020002000000001000000800000012082100002000000100000002001002000010000040100001020010113000070048010800410010000000088000100000000000000011020420000011040001042",
            "miner": "0x0100000000000000000000000000000000000000",
            "mixHash": "0x0000000000000000000000000000000000000000000000000000000000000000",
            "nonce": "0x0000000000000000",
            "number": "0x28dc6b4",
            "parentHash": "0x7c14dbd6685d6f7d3745f8ef5e09db8b56e001d187ae4ab193954992be6d6b30",
            "receiptsRoot": "0x7d194874ddf90a59a8964267bcf84b726e7c34adbecdf066820b681714c0ccf4",
            "sha3Uncles": "0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347",
            "stateRoot": "0xfa51b67bd28e8f1aacca08a4f2c531a6a184196c57503ce9fa5ccc812080ec69",
            "timestamp": "0x65f1919b",
            "totalDifficulty": "0x28dc6b4",
            "transactionsRoot": "0x9dd271555363c817a8a7d3e54ae807dd81d3ac6eb045f2c30ab022be2b5ade92"
        }
    );

    let block = serde_json::from_value::<AvalancheBlock>(json).unwrap();
    block.insert(&pool, &None).await.unwrap();
    block.emit(&redis, &NetworkKind::Avalanche).await.unwrap();

    teardown_test_db(pool, &db_name).await.unwrap();
}

#[tokio::test]
async fn test_insert_and_emit_avalanche_log() {
    let (pool, db_name) = setup_test_db().await.unwrap();
    let redis = setup_test_redis().await;

    let json = serde_json::json!(
        {
            "address": "0x8ad25b0083c9879942a64f00f20a70d3278f6187",
            "blockHash": "0xec173c5460deb9bd52d56d4dcc5cf3f18006fb348757ceb2b654092966d56975",
            "blockNumber": "0x28dc6c4",
            "data": "0x0000000000000000000000000000000000000000000000086365379589af0000",
            "logIndex": "0x10",
            "removed": false,
            "topics": [
              "0x8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925",
              "0x000000000000000000000000b1e392a7c3d644f12869d609784bb52b5798faef",
              "0x000000000000000000000000b4315e873dbcf96ffd0acd8ea43f689d8c20fb30"
            ],
            "transactionHash": "0x4b16d2b0accda9b598dd32362a8fe1c8aa44dbca35a9cb9275785d001d6289b0",
            "transactionIndex": "0x6"
        }
    );

    let log = serde_json::from_value::<AvalancheLog>(json).unwrap();
    log.insert(&pool, &log.core().tx_hash).await.unwrap();
    log.emit(&redis, &NetworkKind::Avalanche).await.unwrap();

    teardown_test_db(pool, &db_name).await.unwrap();
}
