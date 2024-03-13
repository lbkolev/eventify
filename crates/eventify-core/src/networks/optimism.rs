use crate::{networks::NetworkClient, traits::Network};
use eventify_primitives::optimism::{Block, Log};

#[derive(Clone, Debug)]
pub struct Optimism {
    client: NetworkClient,
}

impl Network for Optimism {
    type Block = Block;
    type Log = Log;

    fn new(client: NetworkClient) -> Optimism {
        Optimism { client }
    }

    fn client(&self) -> &NetworkClient {
        &self.client
    }
}
