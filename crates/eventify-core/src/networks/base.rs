use crate::{networks::NetworkClient, traits::Network};
use eventify_primitives::base::{Block, Log};

#[derive(Clone, Debug)]
pub struct Base {
    client: NetworkClient,
}

impl Network for Base {
    type Block = Block;
    type Log = Log;

    fn new(client: NetworkClient) -> Base {
        Base { client }
    }

    fn client(&self) -> &NetworkClient {
        &self.client
    }
}
