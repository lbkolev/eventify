use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use crate::{Auth, Block, Contract, Log, Result, Transaction};

pub trait Insertable: Send + Sync + Debug {}

impl Insertable for Block {}
impl Insertable for Transaction {}
impl Insertable for Log {}
impl Insertable for Contract {}

#[async_trait::async_trait]
pub trait Storage: 'static + Sized + Send + Sync + Debug + Deref + DerefMut + Auth + Clone {
    async fn store_block(&self, block: &Block) -> Result<()>;
    async fn store_transaction(&self, transaction: &Transaction) -> Result<()>;
    async fn store_log(&self, log: &Log) -> Result<()>;
    async fn store_contract(&self, contract: &Contract) -> Result<()>;
}
