use std::sync::Arc;

use alloy_primitives::{Address, BlockNumber, Bloom, Bytes, B256, B64, U256, U64};
use ethers_core::types::Filter;
use jsonrpsee::{
    core::{
        client::{ClientT, Subscription, SubscriptionClientT},
        Serialize,
    },
    rpc_params,
    server::{RpcModule, Server, SubscriptionMessage, TrySendError},
    ws_client::{WsClient, WsClientBuilder},
    PendingSubscriptionSink,
};
use serde::{Deserialize, Deserializer};

use crate::{
    clients::{NodeClient, NodeClientKind},
    NodeClientError,
};

#[async_trait::async_trait]
pub trait NodeClient2: 'static + Clone + Send + Sync {
    fn inner(&self) -> &Arc<WsClient>;
    fn host(&self) -> &String;

    async fn get_block_number(&self) -> Result<BlockNumber, NodeClientError> {
        let s: Result<String, NodeClientError> = self
            .inner()
            .request("eth_blockNumber", rpc_params![])
            .await
            .map_err(|_| NodeClientError::LatestBlock);

        if let Ok(s) = s {
            Ok(BlockNumber::from_str_radix(s.trim_start_matches("0x"), 16)?)
        } else {
            Err(NodeClientError::LatestBlock)
        }
    }
}

#[derive(Debug, Default)]
pub enum NodeClientKind2 {
    #[default]
    Eth,
    ZkSync,
    Starkware,
}

#[derive(Debug)]
pub struct CC {
    inner: Arc<WsClient>,
    host: String,
    kind: NodeClientKind2,
}

//impl<C: NodeClient2> CC<C> {
//    pub async fn new(
//        host: String,
//        kind: NodeClientKind,
//    ) -> Result<Self, Box<dyn std::error::Error>> {
//        Ok(Self {
//            inner: Arc::new(WsClientBuilder::default().build(&host).await?),
//            host,
//            kind,
//        })
//    }
//}

#[derive(Debug, Default, serde::Serialize, Deserialize)]
pub struct EthBlock {
    // -- header
    #[serde(rename = "parentHash")]
    pub parent_hash: B256,
    #[serde(rename = "sha3Uncles")]
    pub uncle_hash: B256,
    #[serde(rename = "miner")]
    coinbase: Address,
    #[serde(rename = "stateRoot")]
    root: B256,
    #[serde(rename = "transactionsRoot")]
    tx_hash: B256,
    #[serde(rename = "receiptsRoot")]
    receipt_hash: B256,
    #[serde(rename = "logsBloom")]
    bloom: Option<Bloom>,
    difficulty: U256,
    //#[serde(deserialize_with = "deserialize_hex_string")]
    number: Option<U64>,
    #[serde(rename = "gasLimit")]
    gas_limit: U256,
    #[serde(rename = "gasUsed")]
    gas_used: U256,
    #[serde(rename = "timestamp")]
    time: U256,
    #[serde(rename = "extraData")]
    extra: Bytes,
    #[serde(rename = "mixHash")]
    mix_digest: B256,
    nonce: Option<B64>,

    /// added by EIP-1559
    #[serde(rename = "baseFeePerGas")]
    base_fee: Option<U256>,

    /// added by EIP-4788
    #[serde(rename = "parentBeaconBlockRoot")]
    parent_beacon_root: Option<B256>,

    /// added by EIP-4844
    #[serde(rename = "blobGasUsed")]
    blob_gas_used: Option<U256>,

    /// added by EIP-4844
    #[serde(rename = "blobGasUsed")]
    excess_blob_gas: Option<U256>,

    /// added by EIP-4895
    #[serde(rename = "withdrawalsHash")]
    withdrawals_hash: Option<B256>,
    // --

    // -- body
    // list of tx hashes
    transactions: Vec<B256>,
    hash: Option<B256>,
    // --
}

#[derive(Debug, Default, serde::Serialize, Deserialize)]
pub struct EthTransaction {
    #[serde(rename = "blockHash")]
    block_hash: Option<B256>,
    #[serde(rename = "blockNumber")]
    block_number: Option<U64>,
    from: Address,
    gas: U256,
    #[serde(rename = "gasPrice")]
    gas_price: U256,
    hash: B256,
    input: Bytes,
    nonce: U256,
    to: Address,
    #[serde(rename = "transactionIndex")]
    transaction_index: Option<U64>,
    value: U256,
    v: U256,
    r: U256,
    s: U256,
}

#[derive(Debug, Default, serde::Serialize, Deserialize)]
struct TransactionResponse {
    transactions: Vec<EthTransaction>,
}

#[derive(Debug, Default, serde::Serialize, Deserialize)]
pub struct EthLog {
    removed: bool,
    #[serde(rename = "logIndex")]
    log_index: U64,
    #[serde(rename = "transactionIndex")]
    transaction_index: Option<U64>,
    #[serde(rename = "transactionHash")]
    transaction_hash: Option<B256>,
    #[serde(rename = "blockHash")]
    block_hash: Option<B256>,
    #[serde(rename = "blockNumber")]
    block_number: Option<U64>,
    address: Address,
    data: Bytes,
    topics: Vec<Option<B256>>,
}

//#[derive(Debug, Default, serde::Serialize, Deserialize)]
//pub struct Filter {
//    #[serde(rename = "fromBlock")]
//    from_block: String,
//
//    #[serde(rename = "toBlock")]
//    to_block: String,
//}

fn deserialize_hex_string<'de, D>(deserializer: D) -> Result<BlockNumber, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    u64::from_str_radix(s.trim_start_matches("0x"), 16).map_err(serde::de::Error::custom)
}

pub struct Provider<P> {
    client: P,
    host: String,
}

#[derive(Clone, Debug)]
pub struct Eth {
    inner: Arc<WsClient>,
    host: String,
}

impl Eth {
    pub async fn new(host: String) -> Result<Self, NodeClientError> {
        Ok(Self {
            inner: Arc::new(
                WsClientBuilder::default()
                    .build(&host)
                    .await
                    .map_err(|_| NodeClientError::Connect)?,
            ),
            host,
        })
    }

    async fn get_block_number(&self) -> Result<BlockNumber, NodeClientError> {
        let s: Result<String, NodeClientError> = self
            .inner
            .request("eth_blockNumber", rpc_params![])
            .await
            .map_err(|_| NodeClientError::LatestBlock);

        if let Ok(s) = s {
            Ok(BlockNumber::from_str_radix(s.trim_start_matches("0x"), 16)?)
        } else {
            Err(NodeClientError::LatestBlock)
        }
    }

    // returns block without transactions
    pub async fn get_lightweight_block(&self, n: BlockNumber) -> Result<EthBlock, NodeClientError> {
        self.inner
            .request(
                "eth_getBlockByNumber",
                rpc_params![format!("0x{:x}", n), false],
            )
            .await
            .map_err(|_| NodeClientError::Block(n))
    }

    pub async fn get_transactions(
        &self,
        n: BlockNumber,
    ) -> Result<Vec<EthTransaction>, NodeClientError> {
        let r: TransactionResponse = self
            .inner
            .request(
                "eth_getBlockByNumber",
                rpc_params![format!("0x{:x}", n), true],
            )
            .await
            .map_err(|_| NodeClientError::Transactions(n))?;

        Ok(r.transactions)
    }

    async fn get_logs(&self, filter: Filter) -> Result<Vec<EthLog>, NodeClientError> {
        self.inner
            .request("eth_getLogs", rpc_params!(filter))
            .await
            .map_err(|_| NodeClientError::Logs("tmp".to_string()))
    }
}

#[derive(Clone, Debug)]
pub struct Zksync {
    inner: Arc<WsClient>,
    host: String,
}

impl Zksync {
    pub async fn new(host: String) -> Result<Self, NodeClientError> {
        Ok(Self {
            inner: Arc::new(
                WsClientBuilder::default()
                    .build(&host)
                    .await
                    .map_err(|_| NodeClientError::Connect)?,
            ),
            host,
        })
    }

    async fn get_block_number(&self) -> Result<BlockNumber, NodeClientError> {
        let s: Result<String, NodeClientError> = self
            .inner
            .request("eth_blockNumber", rpc_params![])
            .await
            .map_err(|_| NodeClientError::LatestBlock);

        if let Ok(s) = s {
            Ok(BlockNumber::from_str_radix(s.trim_start_matches("0x"), 16)?)
        } else {
            Err(NodeClientError::LatestBlock)
        }
    }
}

#[cfg(test)]
mod tests {
    use ethers_core::types::H256;

    use super::*;

    #[tokio::test]
    async fn test_eth_get_block_number() {
        let client = Eth::new("wss://eth.llamarpc.com".to_string())
            .await
            .unwrap();

        assert!(client.get_block_number().await.is_ok());
    }

    #[tokio::test]
    async fn test_eth_get_lightweight_block() {
        let client = Eth::new("wss://eth.llamarpc.com".to_string())
            .await
            .unwrap();

        let block = client.get_lightweight_block(1911151).await;
        println!("{:#?}", block);
        assert!(block.is_ok());
    }

    #[tokio::test]
    async fn test_eth_get_transactions() {
        let client = Eth::new("wss://eth.llamarpc.com".to_string())
            .await
            .unwrap();

        let tx = client.get_transactions(1911151).await;
        println!("{:#?}", tx);
        assert!(tx.is_ok());
    }

    #[tokio::test]
    async fn test_eth_get_log() {
        let client = Eth::new("wss://eth.llamarpc.com".to_string())
            .await
            .unwrap();

        let filter = Filter::new().select(1911151..1911152);
        let logs = client.get_logs(filter).await;

        println!("{:#?}", logs);
        assert!(logs.is_ok());
    }

    #[tokio::test]
    async fn test_eth_latest_block() {
        let client = Eth::new("wss://eth.llamarpc.com".to_string())
            .await
            .unwrap();

        let block = client
            .get_lightweight_block(client.get_block_number().await.unwrap())
            .await;
        println!("{:#?}", block);
        assert!(block.is_ok());
    }

    #[tokio::test]
    async fn test_zksync_get_block_number() {
        let client = Eth::new("wss://sepolia.era.zksync.dev/ws".to_string())
            .await
            .unwrap();

        assert!(client.get_block_number().await.is_ok());
    }
}
