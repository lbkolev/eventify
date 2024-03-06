use alloy_primitives::B256;
use reconnecting_jsonrpsee_ws_client::{rpc_params, RpcError, Subscription};

use crate::{networks::NetworkClient, traits::Network};
use eventify_primitives::networks::ethereum::{EthBlock, EthLog, EthTransaction};

#[derive(Clone, Debug)]
pub struct Eth {
    client: NetworkClient,
}

impl Network for Eth {
    type Block = EthBlock<EthTransaction>;
    type LightBlock = EthBlock<B256>;
    type Transaction = EthTransaction;
    type Log = EthLog;

    fn new(client: NetworkClient) -> Eth {
        Eth { client }
    }

    async fn sub_blocks(&self) -> Result<Subscription, RpcError> {
        self.client
            .inner
            .subscribe(
                "eth_subscribe".to_string(),
                rpc_params!["newHeads"],
                "eth_unsubscribe".to_string(),
            )
            .await
    }

    async fn sub_txs(&self) -> Result<Subscription, RpcError> {
        self.client
            .inner
            .subscribe(
                "eth_subscribe".to_string(),
                rpc_params!["newPendingTransactions"],
                "eth_unsubscribe".to_string(),
            )
            .await
    }

    async fn sub_logs(&self) -> Result<Subscription, RpcError> {
        self.client
            .inner
            .subscribe(
                "eth_subscribe".to_string(),
                rpc_params!["logs"],
                "eth_unsubscribe".to_string(),
            )
            .await
    }
}
