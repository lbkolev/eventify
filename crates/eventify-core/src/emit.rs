#![allow(dead_code)]
const CHANNEL_ETH_BLOCKS: &str = "eth:blocks";
const CHANNEL_ETH_TXS: &str = "eth:txs";
const CHANNEL_ETH_EVENTS: &str = "eth:events";
const CHANNEL_ETH_ERC_TRANSFER: &str = "eth:erc-transfer";
const CHANNEL_ETH_ERC_APPROVAL: &str = "eth:erc-approval";
const CHANNELETH_ERC_APPROVAL_FOR_ALL: &str = "eth:erc-approval-for-all";

const CHANNEL_ZKSYNC_BLOCKS: &str = "zksync:blocks";
const CHANNEL_ZKSYNC_TXS: &str = "zksync:txs";
const CHANNEL_ZKSYNC_EVENTS: &str = "zksync:events";

#[cfg(test)]
mod tests {
    use super::*;
    use redis::{Commands, RedisResult};
    use serde::{Deserialize, Serialize};

    #[test]
    fn test_redis() {
        let client = redis::Client::open("redis://127.0.0.1/").unwrap();
        let mut con = client.get_connection().unwrap();

        #[derive(Debug, Deserialize, Serialize)]
        pub struct MyStruct {
            field1: String,
            field2: i32,
        }

        let my_data = MyStruct {
            field1: String::from("rand"),
            field2: 11,
        };

        let data = serde_json::to_string(&my_data).unwrap();

        let _: () = con.publish("rand", data).unwrap();
    }
}
