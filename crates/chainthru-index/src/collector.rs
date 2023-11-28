use ethers_core::{
    types::{Address, BlockNumber, Filter, Log, ValueOrArray, H256},
    utils::keccak256,
};
use ethers_providers::{Middleware, Provider, SubscriptionStream, Ws};
use futures::{stream, stream::SelectAll};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Collector is responsible for connecting to a node and subscribing to events
/// that match a set of criterias.
///
/// The collector will return a stream of events that match the criterias.
pub struct Collector {
    client: Arc<Provider<Ws>>,
    pub criterias: Vec<Criteria>,
}

impl Collector {
    pub async fn new(host: &str, criterias: Vec<impl Into<Criteria>>) -> Self {
        Self {
            client: Arc::new(
                Provider::<Ws>::connect_with_reconnects(host, 10)
                    .await
                    .unwrap(),
            ),
            criterias: criterias
                .into_iter()
                .map(|criteria| criteria.into())
                .collect(),
        }
    }

    pub async fn streams(&self) -> SelectAll<SubscriptionStream<Ws, Log>> {
        let mut streams: Vec<SubscriptionStream<'_, Ws, Log>> = vec![];

        for criteria in self.criterias.iter() {
            let filter = Filter::new()
                .from_block(BlockNumber::Latest)
                .topic0(ValueOrArray::Array(
                    criteria
                        .events
                        .clone()
                        .into_iter()
                        .map(|event| H256::from(keccak256(event)))
                        .collect(),
                ))
                .address(ValueOrArray::Array(criteria.addresses.clone()));

            let stream = self.client.subscribe_logs(&filter).await.unwrap();
            streams.push(stream);
        }

        stream::select_all(streams)
    }
}

/// Criteria is a set of events and addresses that the collector will subscribe to.
#[derive(Debug, Serialize, Deserialize)]
pub struct Criteria {
    pub name: String,
    pub events: Vec<String>,
    pub addresses: Vec<Address>,
}
