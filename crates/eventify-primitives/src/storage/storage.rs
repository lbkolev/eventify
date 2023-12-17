use async_trait::async_trait;
use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use crate::{Auth, Contract, IndexedBlock, IndexedLog, IndexedTransaction, Result};

pub trait Insertable: Send + Sync + Debug {}

impl Insertable for IndexedBlock {}
impl Insertable for IndexedTransaction {}
impl Insertable for IndexedLog {}
impl Insertable for Contract {}

#[async_trait]
pub trait Storage: 'static + Sized + Send + Sync + Debug + Deref + DerefMut + Auth + Clone {
    async fn insert_block(&self, block: &IndexedBlock) -> Result<()>;

    async fn insert_transaction(&self, transaction: &IndexedTransaction) -> Result<()>;

    async fn insert_log(&self, log: &IndexedLog) -> Result<()>;

    async fn insert_contract(&self, contract: &Contract) -> Result<()>;
}
