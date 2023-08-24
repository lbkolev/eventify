use crate::Result;
use sqlx::ConnectOptions;

pub trait Auth<T: ConnectOptions> {
    fn authenticate(&self, url: impl Into<String>) -> Result<T>;
}
