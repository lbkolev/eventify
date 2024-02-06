#![allow(dead_code)]

use eventify_primitives::networks::{NetworkKind, ResourceKind};

use redis::Commands;
use serde::Serialize;

use crate::{eth::Eth, traits::Emit};

impl Emit<Eth> for redis::Client {
    fn publish<T: Serialize>(
        &self,
        network: &NetworkKind,
        resource: &ResourceKind,
        message: &T,
    ) -> eyre::Result<()> {
        let mut con = self.get_connection()?;
        let channel = format!("{}:{}", network, resource);

        con.publish(channel, serde_json::to_string(message)?)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use redis::Commands;
    use serde::{Deserialize, Serialize};

    #[test]
    fn test_redis() {
        let client = redis::Client::open("redis://127.0.0.1/").unwrap();
        let mut con = client.get_connection().unwrap();

        #[derive(Debug, Deserialize, Serialize)]
        pub(crate) struct MyStruct {
            field1: String,
            field2: i32,
        }

        let my_data = MyStruct {
            field1: String::from("rand"),
            field2: 11,
        };

        let data = serde_json::to_string(&my_data).unwrap();

        let _: () = con.publish("rand", data).unwrap();
    }
}
