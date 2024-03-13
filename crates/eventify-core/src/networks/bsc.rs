use crate::{networks::NetworkClient, traits::Network};
use eventify_primitives::bsc::{Block, Log};

#[derive(Clone, Debug)]
pub struct Bsc {
    client: NetworkClient,
}

impl Network for Bsc {
    type Block = Block;
    type Log = Log;

    fn new(client: NetworkClient) -> Bsc {
        Bsc { client }
    }

    fn client(&self) -> &NetworkClient {
        &self.client
    }
}
