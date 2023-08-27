#![allow(clippy::option_map_unit_fn)]

use std::ops::{Deref, DerefMut};

use sqlx::{pool::PoolOptions, Pool};
use web3::types::H64;

use crate::{storage::Auth, storage::Storage, Error, Result};

#[derive(Debug)]
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
        let sql = "INSERT INTO public.block
            (hash, parent_hash, uncles_hash, author, state_root, transactions_root, receipts_root, number, gas_used, gas_limit, base_fee_per_gas, difficulty, total_difficulty, transactions, size, nonce)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)
            ON CONFLICT DO NOTHING";

        let mut number_slice = [0u8; 8];
        block.number.map(|v| v.to_big_endian(&mut number_slice));

        let mut gas_used_slice = [0u8; 32];
        block.gas_used.map(|v| v.to_big_endian(&mut gas_used_slice));

        let mut gas_limit_slice = [0u8; 32];
        block
            .gas_limit
            .map(|v| v.to_big_endian(&mut gas_limit_slice));

        let mut base_fee_per_gas_slice = [0u8; 32];
        block
            .base_fee_per_gas
            .map(|v| v.to_big_endian(&mut base_fee_per_gas_slice));

        let mut difficulty_slice = [0u8; 32];
        block
            .difficulty
            .map(|v| v.to_big_endian(&mut difficulty_slice));

        let mut total_difficulty_slice = [0u8; 32];
        block
            .total_difficulty
            .map(|v| v.to_big_endian(&mut total_difficulty_slice));

        let mut size_slice = [0u8; 32];
        block.size.map(|v| v.to_big_endian(&mut size_slice));

        sqlx::query(sql)
            .bind(block.hash.as_ref().map(|h| h.as_bytes()))
            .bind(block.parent_hash.as_ref().map(|h| h.as_bytes()))
            .bind(block.uncles_hash.as_ref().map(|h| h.as_bytes()))
            .bind(block.author.as_ref().map(|h| h.as_bytes()))
            .bind(block.state_root.as_ref().map(|h| h.as_bytes()))
            .bind(block.transactions_root.as_ref().map(|h| h.as_bytes()))
            .bind(block.receipts_root.as_ref().map(|h| h.as_bytes()))
            .bind(number_slice)
            .bind(gas_used_slice)
            .bind(gas_limit_slice)
            .bind(base_fee_per_gas_slice)
            .bind(difficulty_slice)
            .bind(total_difficulty_slice)
            .bind(block.transactions.unwrap_or(0) as i32)
            .bind(size_slice)
            .bind(block.nonce.unwrap_or(H64::zero()).as_bytes())
            .execute(&self.inner)
            .await?;

        Ok(())
    }

    async fn insert_contract(&self, contract: &crate::contract::Contract) -> Result<()> {
        let sql = "INSERT INTO public.contract
            (contract_addr, transaction_hash, _from, input)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT DO NOTHING";

        sqlx::query(sql)
            .bind(contract.address.as_bytes())
            .bind(contract.transaction_hash.as_bytes())
            .bind(contract.from.as_bytes())
            .bind(&contract.input.0)
            .execute(&self.inner)
            .await?;

        Ok(())
    }

    async fn insert_transaction(
        &self,
        transaction: &crate::transaction::IndexedTransaction,
    ) -> Result<()> {
        let sql = "INSERT INTO public.transaction
            (hash, _from, _to, input)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT DO NOTHING";

        sqlx::query(sql)
            .bind(transaction.hash.as_bytes())
            .bind(transaction.from.as_ref().map(|x| x.as_bytes()))
            .bind(transaction.to.as_ref().map(|x| x.as_bytes()))
            .bind(&transaction.input.0)
            .execute(&self.inner)
            .await?;

        Ok(())
    }

    async fn insert_transfer(&self, transfer: &crate::func::Transfer) -> Result<()> {
        let sql = "
            INSERT INTO contract_fn.transfer (contract_addr, transaction_hash, transaction_sender, _to, _value)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT DO NOTHING
            ";

        let mut value_slice = [0u8; 32];
        transfer._value.to_big_endian(&mut value_slice);

        sqlx::query(sql)
            .bind(transfer.contract_addr().as_bytes())
            .bind(transfer.transaction_hash().as_bytes())
            .bind(transfer.transaction_sender().as_bytes())
            .bind(transfer._to.as_bytes())
            .bind(value_slice)
            .execute(&self.inner)
            .await?;

        Ok(())
    }

    async fn insert_transfer_from(&self, transfer_from: &crate::func::TransferFrom) -> Result<()> {
        let sql = "
            INSERT INTO contract_fn.transfer_from (contract_addr, transaction_hash, transaction_sender, _from, _to, _value)
            VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT DO NOTHING
            ";

        let mut value_slice = [0u8; 32];
        transfer_from._value.to_big_endian(&mut value_slice);

        sqlx::query(sql)
            .bind(transfer_from.contract_addr().as_bytes())
            .bind(transfer_from.transaction_hash().as_bytes())
            .bind(transfer_from.transaction_sender().as_bytes())
            .bind(transfer_from._from.as_bytes())
            .bind(transfer_from._to.as_bytes())
            .bind(value_slice)
            .execute(&self.inner)
            .await?;

        Ok(())
    }

    async fn insert_approve(&self, approve: &crate::func::Approve) -> Result<()> {
        let sql = "
            INSERT INTO contract_fn.approve (contract_addr, transaction_hash, transaction_sender, _spender, _value)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT DO NOTHING
            ";

        let mut value_slice = [0u8; 32];
        approve._value.to_big_endian(&mut value_slice);

        sqlx::query(sql)
            .bind(approve.contract_addr().as_bytes())
            .bind(approve.transaction_hash().as_bytes())
            .bind(approve.transaction_sender().as_bytes())
            .bind(approve._spender.as_bytes())
            .bind(value_slice)
            .execute(&self.inner)
            .await?;

        Ok(())
    }
}
