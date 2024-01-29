#![allow(dead_code)]

use redis::{Commands, RedisError};

pub trait Emit: 'static + Clone + Send + Sync {
    fn publish(&self, channel: &str, message: String) -> Result<(), EmitError>;
}

impl Emit for redis::Client {
    fn publish(&self, channel: &str, message: String) -> Result<(), EmitError> {
        let mut con = self.get_connection()?;

        con.publish(channel, message)?;
        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum EmitError {
    #[error("Redis publish error: {0}")]
    RedisPublishError(#[from] RedisError),
}

#[cfg(test)]
mod tests {
    use super::*;
    use redis::{Commands, RedisResult};
    use serde::{Deserialize, Serialize};

    #[test]
    fn test_redis() {
        let client = redis::Client::open("redis://127.0.0.1/").unwrap();
        let mut con = client.get_connection().unwrap();

        #[derive(Debug, Deserialize, Serialize)]
        pub struct MyStruct {
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
