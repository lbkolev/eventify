use crate::{networks::NetworkClient, traits::Network};
use eventify_primitives::linea::{Block, Log};

#[derive(Clone, Debug)]
pub struct Linea {
    client: NetworkClient,
}

impl Network for Linea {
    type Block = Block;
    type Log = Log;

    fn new(client: NetworkClient) -> Linea {
        Linea { client }
    }

    fn client(&self) -> &NetworkClient {
        &self.client
    }
}
