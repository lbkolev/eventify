use crate::{networks::NetworkClient, traits::Network};
use eventify_primitives::arbitrum::{Block, Log};

#[derive(Clone, Debug)]
pub struct Arbitrum {
    client: NetworkClient,
}

impl Network for Arbitrum {
    type Block = Block;
    type Log = Log;

    fn new(client: NetworkClient) -> Arbitrum {
        Arbitrum { client }
    }

    fn client(&self) -> &NetworkClient {
        &self.client
    }
}
