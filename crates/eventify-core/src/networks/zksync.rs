use crate::{networks::NetworkClient, traits::Network};
use eventify_primitives::zksync::{Block, Log};

#[derive(Clone, Debug)]
pub struct Zksync {
    client: NetworkClient,
}

impl Network for Zksync {
    type Block = Block;
    type Log = Log;

    fn new(client: NetworkClient) -> Zksync {
        Zksync { client }
    }

    fn client(&self) -> &NetworkClient {
        &self.client
    }
}
