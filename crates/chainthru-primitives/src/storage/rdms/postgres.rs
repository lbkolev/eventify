#![allow(clippy::option_map_unit_fn)]

use std::ops::{Deref, DerefMut};

use sqlx::{pool::PoolOptions, Pool};

use ethers_core::types::{H64, U64};

use crate::{
    storage::{Auth, Storage},
    Error, Result,
};

#[derive(Debug, Clone)]
pub struct Postgres {
    pub inner: Pool<sqlx::postgres::Postgres>,
}

impl Deref for Postgres {
    type Target = Pool<sqlx::postgres::Postgres>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Postgres {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

#[async_trait::async_trait]
impl Auth for Postgres {
    async fn connect(url: &str) -> Self {
        Self {
            inner: PoolOptions::new()
                .acquire_timeout(std::time::Duration::from_secs(2))
                .connect_lazy(url)
                .map_err(Error::from)
                .expect("Failed to connect to Postgres"),
        }
    }

    fn connect_lazy(url: &str) -> Self {
        Self {
            inner: PoolOptions::new()
                .connect_lazy(url)
                .map_err(Error::from)
                .expect("Failed to connect to Postgres"),
        }
    }
}

#[async_trait::async_trait]
impl Storage for Postgres {
    async fn insert_block(&self, block: &crate::IndexedBlock) -> Result<()> {
        let sql = "INSERT INTO public.block (
            hash,
            parent_hash,
            uncles_hash,
            author,
            state_root,
            transactions_root,
            receipts_root,
            number,
            gas_used,
            gas_limit,
            base_fee_per_gas,
            difficulty,
            total_difficulty,
            size,
            nonce
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15
            ) ON CONFLICT DO NOTHING";

        let mut gas_used_slice = [0u8; 32];
        block.gas_used().to_big_endian(&mut gas_used_slice);

        let mut gas_limit_slice = [0u8; 32];
        block.gas_limit().to_big_endian(&mut gas_limit_slice);

        let mut base_fee_per_gas_slice = [0u8; 32];
        block
            .base_fee_per_gas()
            .map(|v| v.to_big_endian(&mut base_fee_per_gas_slice));

        let mut difficulty_slice = [0u8; 32];
        block.difficulty().to_big_endian(&mut difficulty_slice);

        let mut total_difficulty_slice = [0u8; 32];
        block
            .total_difficulty()
            .map(|v| v.to_big_endian(&mut total_difficulty_slice));

        let mut size_slice = [0u8; 32];
        block.size().map(|v| v.to_big_endian(&mut size_slice));

        sqlx::query(sql)
            .bind(block.hash().as_ref().map(|h| h.as_bytes()))
            .bind(block.parent_hash().as_ref())
            .bind(block.uncles_hash().as_ref())
            .bind(block.author().as_ref().map(|h| h.as_bytes()))
            .bind(block.state_root().as_ref())
            .bind(block.transactions_root().as_ref())
            .bind(block.receipts_root().as_ref())
            .bind(block.number().unwrap_or(U64::zero()).as_u64() as i32)
            .bind(gas_used_slice)
            .bind(gas_limit_slice)
            .bind(base_fee_per_gas_slice)
            .bind(difficulty_slice)
            .bind(total_difficulty_slice)
            //.bind(block.transactions().unwrap_or(0) as i32)
            .bind(size_slice)
            .bind(block.nonce().unwrap_or(H64::zero()).as_bytes())
            .execute(&self.inner)
            .await?;

        log::info!("Inserted block: [{:?}]", block.hash());
        Ok(())
    }

    async fn insert_transaction(&self, tx: &crate::IndexedTransaction) -> Result<()> {
        let sql = "INSERT INTO public.transaction (
            hash,
            nonce,
            block_hash,
            block_number,
            transaction_index,
            _from,
            _to,
            value,
            gas_price,
            gas,
            input,
            v, r, s,
            transaction_type,
            max_fee_per_gas,
            max_priority_fee_per_gas
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17
            ) ON CONFLICT DO NOTHING";

        let mut nonce_slice = [0u8; 32];
        tx.nonce().to_big_endian(&mut nonce_slice);

        let mut value_slice = [0u8; 32];
        tx.value().to_big_endian(&mut value_slice);

        let mut gas_price_slice = [0u8; 32];
        tx.gas_price()
            .map(|v| v.to_big_endian(&mut gas_price_slice));

        let mut gas_slice = [0u8; 32];
        tx.gas().to_big_endian(&mut gas_slice);

        let mut r_slice = [0u8; 32];
        tx.r().to_big_endian(&mut r_slice);

        let mut s_slice = [0u8; 32];
        tx.s().to_big_endian(&mut s_slice);

        let mut max_fee_per_gas_slice = [0u8; 32];
        tx.max_fee_per_gas()
            .map(|v| v.to_big_endian(&mut max_fee_per_gas_slice));

        let mut max_priority_fee_per_gas_slice = [0u8; 32];
        tx.max_priority_fee_per_gas()
            .map(|v| v.to_big_endian(&mut max_priority_fee_per_gas_slice));

        sqlx::query(sql)
            .bind(tx.hash().as_bytes())
            .bind(nonce_slice)
            .bind(tx.block_hash().as_ref().map(|h| h.as_bytes()))
            .bind(tx.block_number().map(|v| v.as_u64() as i32))
            .bind(tx.transaction_index().map(|v| v.as_u64() as i32))
            .bind(tx._from().as_ref())
            .bind(tx.to().as_ref().map(|x| x.as_bytes()))
            .bind(value_slice)
            .bind(gas_price_slice)
            .bind(gas_slice)
            .bind(tx.input().0.as_ref())
            .bind(tx.v().as_u64() as i32)
            .bind(r_slice)
            .bind(s_slice)
            .bind(tx.transaction_type().map(|v| v.as_u64() as i32))
            .bind(max_fee_per_gas_slice)
            .bind(max_priority_fee_per_gas_slice)
            .execute(&self.inner)
            .await?;

        log::debug!("Inserted transaction: [{:?}]", tx.hash());
        Ok(())
    }

    async fn insert_contract(&self, tx: &crate::Contract) -> Result<()> {
        let sql = "INSERT INTO public.contract (
            transaction_hash,
            _from,
            input
            ) VALUES (
                $1, $2, $3
            ) ON CONFLICT DO NOTHING";

        sqlx::query(sql)
            .bind(tx.transaction_hash().as_bytes())
            .bind(tx._from().as_ref())
            .bind(tx.input().0.as_ref())
            .execute(&self.inner)
            .await?;

        log::debug!("Inserted contract [{:?}]", tx.transaction_hash());
        Ok(())
    }
}
