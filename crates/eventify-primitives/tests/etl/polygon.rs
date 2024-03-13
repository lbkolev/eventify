use alloy_primitives::B256;

use crate::utils::{setup_test_db, setup_test_redis, teardown_test_db};

use eventify_primitives::{
    networks::{
        polygon::{PolygonBlock, PolygonLog},
        NetworkKind,
    },
    EmitT, InsertT, LogT,
};

#[tokio::test]
async fn test_insert_and_emit_polygon_block() {
    let (pool, db_name) = setup_test_db().await.unwrap();
    let redis = setup_test_redis().await;

    let json = serde_json::json!(
      {
        "parentHash": "0x625d92d91abc1ac1e825741c5a79c875aedd6a2de555ef273bb8ff4236298f17",
        "sha3Uncles": "0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347",
        "miner": "0x0000000000000000000000000000000000000000",
        "stateRoot": "0x33c1e61ec333a03ba3009845b840b2a234a11ef8b624d53c46c16e0708ced5e3",
        "transactionsRoot": "0xddc5c0baa6006e4fc38c8ef50728cf5f1f784785b92ccc4ba232f2ebdb0e1d33",
        "receiptsRoot": "0xe5926e89f9baf23dab58fec0650c536ce4c6122e5f20d303b1b44b872ceae9b6",
        "logsBloom": "0x7dfed93bf867f38bce7fe2999686e9ed0ab546ccb40da23658be72aea7deed3955fb984ceea43f3f554ff49c6f5766cded71f7d1e593f64ea46b72ee29a4698670dcad94ded648bb35da3e2de53d2bfde4ac966d3de6c9fcd16fe0fc418f23dc2e17806efa0102b28073fab547361cd1f18b26c47fb8426dbb31fc7dcc7c10d95d75c66f9c4eb2816529559c7401e19a01abced52bcb146835313fdd0a050bb972912e13fde4cab1d1f0de4ff913efc27ce36df8a3f17dccc7776400cc2c6fef2b58dfe747f899b1c3a96b64cb0c74f0f5709e98d89ebe15687bf52ea0b761f5f611bc8501c7dd2910d57af83e31d174a7627ca253da0b976b094aca325c39aa",
        "difficulty": "0x1a",
        "number": "0x3413943",
        "gasLimit": "0x1c7fa6e",
        "gasUsed": "0x140d2f6",
        "timestamp": "0x65f187c1",
        "extraData": "0xd88301020783626f7289676f312e32302e3134856c696e757800000000000000f4c010f7bf01687e036559bbf5609da2fadc92729a83c6b1facd26f519af88643eb0be25490a8372cfb5c26ab7b1d4b56dba29ba8a48629fc91160f7701b74ae00",
        "mixHash": "0x0000000000000000000000000000000000000000000000000000000000000000",
        "nonce": "0x0000000000000000",
        "baseFeePerGas": "0x159f8c6bd8",
        "withdrawalsRoot": null,
        "blobGasUsed": null,
        "excessBlobGas": null,
        "parentBeaconBlockRoot": null,
        "hash": "0xedfb0bdc68e649527a4f90b4b5c897531018d857f17bee672db538f77cb43f2c"
      }
    );

    let block = serde_json::from_value::<PolygonBlock>(json).unwrap();
    block.insert(&pool, &None).await.unwrap();
    block.emit(&redis, &NetworkKind::Polygon).await.unwrap();

    teardown_test_db(pool, &db_name).await.unwrap();
}

#[tokio::test]
async fn test_insert_and_emit_polygon_log() {
    let (pool, db_name) = setup_test_db().await.unwrap();
    let redis = setup_test_redis().await;

    let json = serde_json::json!({
      "address": "0x0000000000000000000000000000000000001010",
      "blockHash": "0xe582012fe151d0ccfbca2619fe9cbd629ddefa26c2ba6db223ec8ecb85a784a0",
      "blockNumber": "0x3409f05",
      "data": "0x0000000000000000000000000000000000000000000000000001e81bd9119818000000000000000000000000000000000000000000000000023f6593bd2f0e9800000000000000000000000000000000000000000002d43477d44efc1390e1a3000000000000000000000000000000000000000000000000023d7d77e41d768000000000000000000000000000000000000000000002d43477d63717eca279bb",
      "logIndex": "0x191",
      "removed": false,
      "topics": [
        "0x4dfe1bbbcf077ddc3e01291eea2d5c70c2b422b415d95645b9adcfd678cb1d63",
        "0x0000000000000000000000000000000000000000000000000000000000001010",
        "0x000000000000000000000000b3cd7d425a16c01ff429e3f7866fe2c061e8a190",
        "0x0000000000000000000000007c7379531b2aee82e4ca06d4175d13b9cbeafd49"
      ],
      "transactionHash": "0x5852aa084db7c9d259094dd921f0afaceea728fd9de293ea09617ea5f99416b8",
      "transactionIndex": "0x78"
    });

    let log = serde_json::from_value::<PolygonLog>(json).unwrap();
    log.insert(&pool, &None).await.unwrap();
    log.emit(&redis, &NetworkKind::Polygon).await.unwrap();

    teardown_test_db(pool, &db_name).await.unwrap();
}
