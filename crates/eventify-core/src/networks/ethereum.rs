use crate::{networks::NetworkClient, traits::Network};
use eventify_primitives::ethereum::{Block, Log};

#[derive(Clone, Debug)]
pub struct Eth {
    client: NetworkClient,
}

impl Network for Eth {
    type Block = Block;
    type Log = Log;

    fn new(client: NetworkClient) -> Eth {
        Eth { client }
    }

    fn client(&self) -> &NetworkClient {
        &self.client
    }
}
