pub mod pg;
pub use pg::Postgres;

use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use crate::Result;
use eventify_primitives::{Block, Contract, Log, Transaction};

#[async_trait::async_trait]
pub trait StorageClient:
    'static + Sized + Send + Sync + Debug + Deref + DerefMut + Auth + Clone
{
    async fn store_block(&self, block: &Block) -> Result<()>;
    async fn store_transaction(&self, transaction: &Transaction) -> Result<()>;
    async fn store_log(&self, log: &Log) -> Result<()>;
    async fn store_contract(&self, contract: &Contract) -> Result<()>;
}

#[async_trait::async_trait]
pub trait Auth {
    async fn connect(url: &str) -> Self;
}
