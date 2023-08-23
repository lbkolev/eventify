use std::ops::{Deref, DerefMut};

use async_trait::async_trait;
use sqlx::{Database, Pool};

mod rdms;

#[async_trait]
pub trait Operations {
    async fn insert_block(
        &self,
        block: &crate::IndexedBlock,
    ) -> Result<(), Box<dyn std::error::Error>>;

    async fn insert_contract(
        &self,
        contract: &crate::contract::Contract,
    ) -> Result<(), Box<dyn std::error::Error>>;

    async fn insert_transaction(
        &self,
        transaction: &crate::transaction::IndexedTransaction,
    ) -> Result<(), Box<dyn std::error::Error>>;

    async fn insert_transfer(
        &self,
        transfer: &crate::func::Transfer,
    ) -> Result<(), Box<dyn std::error::Error>>;

    async fn insert_transfer_from(
        &self,
        transfer_from: &crate::func::TransferFrom,
    ) -> Result<(), Box<dyn std::error::Error>>;

    async fn insert_approve(
        &self,
        approve: &crate::func::Approve,
    ) -> Result<(), Box<dyn std::error::Error>>;
}

pub struct Storage<DB>
where
    DB: 'static + Database + Operations + Sized + Send,
{
    pub db: Pool<DB>,
}

impl<DB> Deref for Storage<DB>
where
    DB: 'static + Database + Operations + Sized + Send,
{
    type Target = Pool<DB>;

    fn deref(&self) -> &Self::Target {
        &self.db
    }
}

impl<DB: Database> DerefMut for Storage<DB>
where
    DB: 'static + Database + Operations + Sized + Send,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.db
    }
}
