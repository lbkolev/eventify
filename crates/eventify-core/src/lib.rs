#![doc = include_str!("../README.md")]
#![warn(missing_debug_implementations, unreachable_pub, rustdoc::all)]
#![deny(unused_must_use, rust_2018_idioms)]

pub mod collector;
pub mod error;
pub mod manager;
pub mod networks;

pub use collector::Collector;
pub use error::Error;
pub use manager::Manager;
pub use networks::{ethereum, zksync};
pub use traits::{Collect as CollectT, Network as NetworkT};

type Result<T> = std::result::Result<T, error::Error>;

mod traits {
    pub trait Collect<E>
    where
        E: std::error::Error + Send + Sync,
    {
        fn stream_blocks(
            &self,
            stop_signal: tokio::sync::watch::Receiver<bool>,
        ) -> impl std::future::Future<Output = Result<(), E>>;
        fn stream_logs(
            &self,
            stop_signal: tokio::sync::watch::Receiver<bool>,
        ) -> impl std::future::Future<Output = Result<(), E>>;
    }

    pub trait Network: 'static + Clone + std::fmt::Debug + Send + Sync {
        type Block: eventify_primitives::BlockT;
        type Log: eventify_primitives::LogT;

        fn new(client: crate::networks::NetworkClient) -> Self;
        fn client(&self) -> &crate::networks::NetworkClient;
        fn sub_blocks(
            &self,
        ) -> impl std::future::Future<
            Output = eyre::Result<
                reconnecting_jsonrpsee_ws_client::Subscription,
                reconnecting_jsonrpsee_ws_client::RpcError,
            >,
        > + Send {
            (*self.client()).subscribe(
                "eth_subscribe".to_string(),
                reconnecting_jsonrpsee_ws_client::rpc_params!["newHeads"],
                "eth_unsubscribe".to_string(),
            )
        }
        fn sub_logs(
            &self,
        ) -> impl std::future::Future<
            Output = eyre::Result<
                reconnecting_jsonrpsee_ws_client::Subscription,
                reconnecting_jsonrpsee_ws_client::RpcError,
            >,
        > + Send {
            (*self.client()).subscribe(
                "eth_subscribe".to_string(),
                reconnecting_jsonrpsee_ws_client::rpc_params!["logs", {}],
                "eth_unsubscribe".to_string(),
            )
        }
    }
}
