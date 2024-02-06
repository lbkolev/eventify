use alloy_primitives::{BlockNumber, B256};

use crate::{networks::NetworkClient, traits::Network, NetworkError};
use eventify_primitives::networks::eth::{Criteria, EthBlock, EthLog, EthTransaction};
use reconnecting_jsonrpsee_ws_client::{rpc_params, Subscription};

#[derive(Clone, Debug)]
pub struct Eth {
    client: NetworkClient,
}

impl Eth {
    pub fn new(client: NetworkClient) -> Eth {
        Eth { client }
    }
}

impl Network for Eth {
    type Block = EthBlock<EthTransaction>;
    type LightBlock = EthBlock<B256>;
    type Transaction = EthTransaction;
    type Log = EthLog;

    async fn get_block_number(&self) -> Result<BlockNumber, NetworkError> {
        const METHOD: &str = "eth_blockNumber";

        let r = self
            .client
            .inner
            .request(METHOD.into(), rpc_params![])
            .await
            .map_err(|e| NetworkError::GetLatestBlockFailed { err: e.to_string() })?;

        let r = serde_json::from_str::<String>(r.get())
            .map_err(|e| NetworkError::DeserializationFailed { err: e.to_string() });
        match r {
            Ok(r) => Ok(BlockNumber::from_str_radix(r.trim_start_matches("0x"), 16)?),
            Err(e) => Err(e),
        }
    }

    async fn get_block(&self, n: BlockNumber) -> Result<Self::LightBlock, NetworkError> {
        const METHOD: &str = "eth_getBlockByNumber";

        let r = self
            .client
            .inner
            .request(METHOD.into(), rpc_params![format!("0x{:x}", n), false])
            .await
            .map_err(|e| NetworkError::GetBlockFailed {
                n,
                err: e.to_string(),
            })?;

        serde_json::from_str::<Self::LightBlock>(r.get())
            .map_err(|e| NetworkError::DeserializationFailed { err: e.to_string() })
    }

    async fn get_transactions(
        &self,
        n: BlockNumber,
    ) -> Result<Vec<Self::Transaction>, NetworkError> {
        const METHOD: &str = "eth_getBlockByNumber";

        let r = self
            .client
            .inner
            .request(METHOD.into(), rpc_params![format!("0x{:x}", n), true])
            .await
            .map_err(|e| NetworkError::GetTransactionsFailed {
                n,
                err: e.to_string(),
            })?;

        let r = serde_json::from_str::<Self::Block>(r.get())
            .map_err(|e| NetworkError::DeserializationFailed { err: e.to_string() });

        match r {
            Ok(r) => Ok(r.transactions.unwrap_or_default()),
            Err(e) => Err(e),
        }
    }

    async fn get_logs(&self, filter: &Criteria) -> Result<Vec<Self::Log>, NetworkError> {
        const METHOD: &str = "eth_getLogs";

        let r = self
            .client
            .inner
            .request(METHOD.into(), rpc_params!(filter))
            .await
            .map_err(|e| NetworkError::GetLogsFailed { err: e.to_string() })?;

        serde_json::from_str::<Vec<Self::Log>>(r.get())
            .map_err(|e| NetworkError::DeserializationFailed { err: e.to_string() })
    }

    async fn sub_blocks(&self) -> Result<Subscription, NetworkError> {
        self.client
            .inner
            .subscribe(
                "eth_subscribe".to_string(),
                rpc_params!["newHeads"],
                "eth_unsubscribe".to_string(),
            )
            .await
            .map_err(|e| NetworkError::BlockSubscriptionFailed {
                sub: "eth_subscribe".into(),
                params: "newHeads".into(),
                err: e.to_string(),
            })
    }

    async fn sub_logs(&self) -> Result<Subscription, NetworkError> {
        self.client
            .inner
            .subscribe(
                "eth_subscribe".to_string(),
                rpc_params!["logs"],
                "eth_unsubscribe".to_string(),
            )
            .await
            .map_err(|e| NetworkError::LogSubscriptionFailed {
                sub: "eth_subscribe".into(),
                params: "logs".into(),
                err: e.to_string(),
            })
    }
}
