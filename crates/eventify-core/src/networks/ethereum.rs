use alloy_primitives::B256;

use crate::{networks::NetworkClient, traits::Network};
use eventify_primitives::networks::ethereum::{EthBlock, EthLog, EthTransaction};

use jsonrpsee::{
    core::client::{Error as RpcError, Subscription, SubscriptionClientT},
    rpc_params,
};

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

    async fn sub_blocks(&self) -> Result<Subscription<Self::LightBlock>, RpcError> {
        self.client
            .inner
            .subscribe("eth_subscribe", rpc_params!["newHeads"], "eth_unsubscribe")
            .await
    }

    async fn sub_txs(&self) -> Result<Subscription<B256>, RpcError> {
        self.client
            .inner
            .subscribe(
                "eth_subscribe",
                rpc_params!["newPendingTransactions"],
                "eth_unsubscribe",
            )
            .await
    }

    async fn sub_logs(&self) -> Result<Subscription<Self::Log>, RpcError> {
        self.client
            .inner
            .subscribe("eth_subscribe", rpc_params!["logs"], "eth_unsubscribe")
            .await
    }
}
