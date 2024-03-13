use crate::{networks::NetworkClient, traits::Network};
use eventify_primitives::polygon::{Block, Log};

#[derive(Clone, Debug)]
pub struct Polygon {
    client: NetworkClient,
}

impl Network for Polygon {
    type Block = Block;
    type Log = Log;

    fn new(client: NetworkClient) -> Polygon {
        Polygon { client }
    }

    fn client(&self) -> &NetworkClient {
        &self.client
    }
}
