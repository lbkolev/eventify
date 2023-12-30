use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use crate::Result;
use eventify_primitives::{Block, Contract, Log, Transaction};

pub trait Insertable: Send + Sync + Debug {}
impl Insertable for Block {}
impl Insertable for Transaction {}
impl Insertable for Log {}
impl Insertable for Contract {}

#[trait_variant::make(StorageClient: Send)]
pub trait LocalStorageClient:
    Auth + Clone + Debug + Deref + DerefMut + Sized + Sync + 'static
{
    async fn store_block(&self, block: &Block) -> Result<()>;
    async fn store_transaction(&self, transaction: &Transaction) -> Result<()>;
    async fn store_log(&self, log: &Log) -> Result<()>;
    async fn store_contract(&self, contract: &Contract) -> Result<()>;
}

#[trait_variant::make(Auth: Send)]
pub trait LocalAuth {
    /// The derived implementation should create a new connection pool with the given connection URL
    /// and immediately establish one connection.
    fn connect(&mut self, url: &str) -> Self;

    /// The derived implementation should be using this method to create a new pool configuration
    /// and not establish connections until needed.
    fn connect_lazy(&mut self, url: &str) -> Self;
}
