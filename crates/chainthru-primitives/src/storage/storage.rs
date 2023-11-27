use async_trait::async_trait;
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

use crate::Error;

#[async_trait]
pub trait Storage: 'static + Sized + Send + Sync + Debug + Deref + DerefMut {
    fn insert_block<'life0, 'life1, 'async_trait>(
        &'life0 self,
        block: &'life1 crate::IndexedBlock,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = Result<(), Error>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait;

    fn insert_transaction<'life0, 'life1, 'async_trait>(
        &'life0 self,
        transaction: &'life1 crate::IndexedTransaction,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = Result<(), Error>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait;

    fn insert_contract<'life0, 'life1, 'async_trait>(
        &'life0 self,
        transaction: &'life1 crate::Contract,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = Result<(), Error>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait;
}
