#![allow(dead_code)]

use eventify_primitives::network::{NetworkKind, ResourceKind};
use redis::{Commands, RedisError};
use serde::Serialize;

pub trait Emit: 'static + Clone + Send + Sync {
    fn publish<T: Serialize>(
        &self,
        network: &NetworkKind,
        resource: &ResourceKind,
        message: &T,
    ) -> Result<(), EmitError>;
}

impl Emit for redis::Client {
    fn publish<T: Serialize>(
        &self,
        network: &NetworkKind,
        resource: &ResourceKind,
        message: &T,
    ) -> Result<(), EmitError> {
        let mut con = self.get_connection()?;
        let channel = format!("{}:{}", network, resource);

        con.publish(channel, serde_json::to_string(message)?)?;
        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum EmitError {
    #[error("Redis publish error: {0}")]
    RedisPublishError(#[from] RedisError),

    #[error("Serde error: {0}")]
    SerdeError(#[from] serde_json::Error),
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
