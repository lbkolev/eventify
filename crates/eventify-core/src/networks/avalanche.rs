use crate::{networks::NetworkClient, traits::Network};
use eventify_primitives::avalanche::{Block, Log};

#[derive(Clone, Debug)]
pub struct Avalanche {
    client: NetworkClient,
}

impl Network for Avalanche {
    type Block = Block;
    type Log = Log;

    fn new(client: NetworkClient) -> Avalanche {
        Avalanche { client }
    }

    fn client(&self) -> &NetworkClient {
        &self.client
    }
}
