use alloy_primitives::B256;

use crate::utils::{setup_test_db, teardown_test_db};

use eventify_primitives::{
    networks::ethereum::{EthBlock, EthLog, EthTransaction},
    InsertT, LogT,
};

#[tokio::test]
async fn test_insert_block() {
    let (pool, db_name) = setup_test_db().await.unwrap();

    let json = serde_json::json!(
    {
        "baseFeePerGas": "0x7",
        "miner": "0x0000000000000000000000000000000000000001",
        "number": "0x1b4",
        "hash": "0x0e670ec64341771606e55d6b4ca35a1a6b75ee3d5145a99d05921026d1527331",
        "parentHash": "0x9646252be9520f6e71339a8df9c55e4d7619deeb018d2a3f2d21fc165dde5eb5",
        "unclesHash": "0x9646252be9520f6e71339a8df9c55e4d7619deeb018d2a3f2d21fc165dde5eb5",
        "mixHash": "0x1010101010101010101010101010101010101010101010101010101010101010",
        "nonce": "0x0000000000000000",
        "sealFields": [
          "0xe04d296d2460cfb8472af2c5fd05b5a214109c25688d3704aed5484f9a7792f2",
          "0x0000000000000042"
        ],
        "sha3Uncles": "0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347",
        "logsBloom":  "0x0e670ec64341771606e55d6b4ca35a1a6b75ee3d5145a99d05921026d15273310e670ec64341771606e55d6b4ca35a1a6b75ee3d5145a99d05921026d15273310e670ec64341771606e55d6b4ca35a1a6b75ee3d5145a99d05921026d15273310e670ec64341771606e55d6b4ca35a1a6b75ee3d5145a99d05921026d15273310e670ec64341771606e55d6b4ca35a1a6b75ee3d5145a99d05921026d15273310e670ec64341771606e55d6b4ca35a1a6b75ee3d5145a99d05921026d15273310e670ec64341771606e55d6b4ca35a1a6b75ee3d5145a99d05921026d15273310e670ec64341771606e55d6b4ca35a1a6b75ee3d5145a99d05921026d1527331",
        "transactionsRoot": "0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421",
        "receiptsRoot": "0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421",
        "stateRoot": "0xd5855eb08b3387c0af375e9cdb6acfc05eb8f519e419b874b6ff2ffda7ed1dff",
        "difficulty": "0x27f07",
        "totalDifficulty": "0x27f07",
        "extraData": "0x0000000000000000000000000000000000000000000000000000000000000000",
        "size": "0x27f07",
        "gasLimit": "0x9f759",
        "minGasPrice": "0x9f759",
        "gasUsed": "0x9f759",
        "timestamp": "0x54e34e8e",
        "transactions": [],
        "uncles": []
      }
    );

    let block = serde_json::from_value::<EthBlock<B256>>(json).unwrap();
    println!("{:?}", block);
    block.insert(&pool, "eth", &None).await.unwrap();

    teardown_test_db(pool, &db_name).await.unwrap();
}

#[tokio::test]
async fn test_insert_transaction() {
    let (pool, db_name) = setup_test_db().await.unwrap();

    let json = serde_json::json!({
        "blockHash":"0x1d59ff54b1eb26b013ce3cb5fc9dab3705b415a67127a003c3e61eb445bb8df2",
        "blockNumber":"0x5daf3b",
        "from":"0xa7d9ddbe1f17865597fbd27ec712455208b6b76d",
        "gas":"0xc350",
        "gasPrice":"0x4a817c800",
        "hash":"0x88df016429689c079f3b2f6ad39fa052532c56795b733da78a91ebe6a713944b",
        "input":"0x68656c6c6f21",
        "nonce":"0x15",
        "to":"0xf02c1c8e6114b1dbe8937a39260b5b0a374432bb",
        "transactionIndex":"0x41",
        "value":"0xf3dbb76162000",
        "v":"0x25",
        "r":"0x1b5e176d927f8e9ab405058b2d2457392da3e20f328b16ddabcebc33eaac5fea",
        "s":"0x4ba69724e8f69de52f0125ad8b3c5c2cef33019bac3249e2c0a2192766d1721c"
    });

    let tx = serde_json::from_value::<EthTransaction>(json).unwrap();
    println!("{:?}", tx);
    tx.insert(&pool, "eth", &None).await.unwrap();

    teardown_test_db(pool, &db_name).await.unwrap();
}

#[tokio::test]
async fn test_insert_log() {
    let (pool, db_name) = setup_test_db().await.unwrap();

    let json = serde_json::json!(
        {
        "address": "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2",
        "topics": [
            "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef",
            "0x000000000000000000000000a7ca2c8673bcfa5a26d8ceec2887f2cc2b0db22a",
            "0x00000000000000000000000006da0fd433c1a5d7a4faa01111c044910a184553"
        ],
        "data": "0x000000000000000000000000000000000000000000000000007c585087238000",
        "blockHash": "0x6624f87d3435cc938de6442db45e06f23582a7eeddb5ac15126d440db03e75f4",
        "blockNumber": "0x11d389d",
        "transactionHash": "0x933c80c2a18cbf64ec28662991186bd340519eb6974f3d301195b82064329fc8",
        "transactionIndex": "0xd5",
        "logIndex": "0x200",
        "transactionLogIndex": null,
        "logType": null,
        "removed": false
        }
    );

    let log = serde_json::from_value::<EthLog>(json).unwrap();
    println!("{:#?}", log);
    log.insert(&pool, "eth", &log.tx_hash()).await.unwrap();

    teardown_test_db(pool, &db_name).await.unwrap();
}
