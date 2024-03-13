use alloy_primitives::B256;

use crate::utils::{setup_test_db, setup_test_redis, teardown_test_db};

use eventify_primitives::{
    networks::{
        ethereum::{EthBlock, EthLog},
        NetworkKind,
    },
    EmitT, InsertT, LogT,
};

#[tokio::test]
async fn test_insert_and_emit_ethereum_block() {
    let (pool, db_name) = setup_test_db().await.unwrap();
    let redis = setup_test_redis().await;

    let json = serde_json::json!(
        {
            "baseFeePerGas": "0xde15854c5",
            "difficulty": "0x0",
            "extraData": "0x6265617665726275696c642e6f7267",
            "gasLimit": "0x1c9c380",
            "gasUsed": "0xd759a1",
            "hash": "0x2d85ca6abdcdbf998730c7fa551b9bcf7bcaf5306785596c316a85606ada164e",
            "logsBloom": "0xb2a3e21be188032072a042849d710202a00b848a4d281001100ba05a2d906216dd980dc106500090e0805b06d2563ba14e0390259800a9a80132328000ae4ca24c1098b883630f68cc02d80d408663e2101684994fe1280c500d54c68b66a8864c422003a8d30059060c50008a06345922720f0b0809a59a19270496050b0044a8a0f270a92984cd24e821e1e100014eca5a20d595a7ac4e762824536b74d0992a3618365a852b26326e5fcc99502d882540c38af083184a2471ac20308e2811c18870cb1a0033450948504b183ad60979f210203c88301860024b428925600d08f8e44888527020841c95c55a91ea840e08909165bc2840400991508900547c",
            "miner": "0x95222290dd7278aa3ddd389cc1e1d165cc4bafe5",
            "mixHash": "0x21ca58d6d4001b9eec84f546a989707aa634ececfd497c1fd28a7a72369b67e8",
            "nonce": "0x0000000000000000",
            "number": "0x1286aa1",
            "parentHash": "0x8ddde44fc63553b07e71d9a506312c77ae7f07f0276bb3cb0d62d4cd254dfea9",
            "receiptsRoot": "0xf9dbb06ca85b01ec8636e048ec8cbe445cf60b0d40aa603c2629ab3fa2f6d9fa",
            "sha3Uncles": "0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347",
            "stateRoot": "0x0d6a47851b4cd5b57b30beb8694968ef514d8db22036b88835a3f447f9c700b3",
            "timestamp": "0x65f19287",
            "totalDifficulty": "0xc70d815d562d3cfa955",
            "transactionsRoot": "0xa43080abb1e151810454877e208acd755fdbc98758f36bda3f03dbef89ef8c1c",
            "withdrawalsRoot": "0x79611a8705955e193086ea13e5541d26195ceecc90c7cbf9b60509a95f1b0436"
        }
    );

    let block = serde_json::from_value::<EthBlock>(json).unwrap();
    block.insert(&pool, &None).await.unwrap();
    block.emit(&redis, &NetworkKind::Ethereum).await.unwrap();

    teardown_test_db(pool, &db_name).await.unwrap();
}

#[tokio::test]
async fn test_insert_and_emit_ethereum_log() {
    let (pool, db_name) = setup_test_db().await.unwrap();
    let redis = setup_test_redis().await;

    let json = serde_json::json!(
        {
            "address": "0xc059a531b4234d05e9ef4ac51028f7e6156e2cce",
            "blockHash": "0x2b910e0db3024f10e5c277700796ec7881a774220c6caacf5e53c5573e5480b3",
            "blockNumber": "0x1286aa4",
            "data": "0x00000000000000000000000000000000000000000000007dd374aa641fac00000000000000000000000000000000000000000000000000000000000065f192ab",
            "logIndex": "0xf0",
            "removed": false,
            "topics": [
              "0x7fc4727e062e336010f2c282598ef5f14facb3de68cf8195c2f23e1454b2b74e",
              "0x000000000000000000000000d82dfb3ac04c022a814fbf8fbac7ab4074a710e1"
            ],
            "transactionHash": "0x78c44371c0fb3caa19137496b11b8a9b2e5104ed3a1c6334e2032f5bcbb17b53",
            "transactionIndex": "0x99"
        }
    );

    let log = serde_json::from_value::<EthLog>(json).unwrap();
    log.insert(&pool, &log.core().tx_hash).await.unwrap();
    log.emit(&redis, &NetworkKind::Ethereum).await.unwrap();

    teardown_test_db(pool, &db_name).await.unwrap();
}
