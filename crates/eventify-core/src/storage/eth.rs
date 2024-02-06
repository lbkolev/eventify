use alloy_primitives::{Bytes, FixedBytes, U64};
use tracing::debug;

use crate::{
    eth::Eth,
    traits::{Network, Store},
    Error, Storage, StorageError,
};
use eventify_primitives::networks::eth::Contract;

impl Store<Eth> for Storage {
    fn schema_name(&self) -> &str {
        "eth"
    }

    async fn store_block(&self, block: &<Eth as Network>::LightBlock) -> Result<(), Error> {
        let sql = format!(
            r#"INSERT INTO {schema}.block (
                parent_hash,
                uncles_hash,
                coinbase,
                root,
                tx_hash,
                receipt_hash,
                difficulty,
                number,
                gas_limit,
                gas_used,
                time,
                extra,
                mix_digest,
                nonce,
                base_fee,
                parent_beacon_root,
                blob_gas_used,
                excess_blob_gas,
                withdraws_hash,
                hash
                ) VALUES (
                    $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20
                ) ON CONFLICT DO NOTHING"#,
            schema = self.schema_name()
        );

        sqlx::query(&sql)
            .bind(block.parent_hash.as_slice())
            .bind(block.uncle_hash.as_slice())
            .bind(block.coinbase.as_slice())
            .bind(block.root.as_slice())
            .bind(block.tx_hash.as_slice())
            .bind(block.receipt_hash.as_slice())
            .bind(block.difficulty.as_le_slice())
            .bind(block.number.map(|v| v.to::<i64>()))
            .bind(block.gas_limit.as_le_slice())
            .bind(block.gas_used.as_le_slice())
            .bind(block.time.to::<i64>())
            .bind(block.extra.to_vec())
            .bind(block.mix_digest.as_slice())
            .bind(block.nonce.as_ref().map(|v| v.as_slice()))
            .bind(block.base_fee.map(|v| v.to::<i64>()))
            .bind(block.parent_beacon_root.as_ref().map(|v| v.as_slice()))
            .bind(block.blob_gas_used.map(|v| v.to::<i64>()))
            .bind(block.excess_blob_gas.map(|v| v.to::<i64>()))
            .bind(block.withdrawals_hash.as_ref().map(|v| v.as_slice()))
            .bind(block.hash.as_ref().map(|v| v.as_slice()))
            .execute(&self.inner)
            .await
            .map_err(|e| StorageError::StoreBlockFailed {
                hash: block.hash.expect("unable to get block hash"),
                err: e.to_string(),
            })?;

        debug!(target: "eventify::core::store::block", hash=?block.hash, number=?block.number);
        Ok(())
    }

    async fn store_transaction(
        &self,
        tx: &<Eth as Network>::Transaction,
    ) -> Result<(), crate::Error> {
        let sql = format!(
            r#"INSERT INTO {schema}.transaction (
            block_hash,
            block_number,
            "from",
            gas,
            gas_price,
            hash,
            input,
            nonce,
            "to",
            transaction_index,
            value,
            v, r, s) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14
            ) ON CONFLICT DO NOTHING"#,
            schema = self.schema_name()
        );

        sqlx::query(&sql)
            .bind(tx.block_hash.as_ref().map(|v| v.as_slice()))
            .bind(tx.block_number.map(|v| v.to::<i64>()))
            .bind(tx.from.as_slice())
            .bind(tx.gas.as_le_slice())
            .bind(tx.gas_price.as_le_slice())
            .bind(tx.hash.as_slice())
            .bind(tx.input.to_vec())
            .bind(tx.nonce.as_le_slice())
            .bind(tx.to.as_ref().map(|v| v.as_slice()))
            .bind(tx.transaction_index.map(|v| v.to::<i64>()))
            .bind(tx.value.as_le_slice())
            .bind(tx.v.as_le_slice())
            .bind(tx.r.as_le_slice())
            .bind(tx.s.as_le_slice())
            .execute(&self.inner)
            .await
            .map_err(|e| StorageError::StoreTransactionFailed {
                hash: tx.hash,
                err: e.to_string(),
            })?;

        debug!(target: "eventify::core::store::transaction", hash=?tx.hash);
        Ok(())
    }

    async fn store_log(&self, log: &<Eth as Network>::Log) -> Result<(), Error> {
        let sql = format!(
            r#"INSERT INTO {schema}.log (
            address,
            topic0,
            topic1,
            topic2,
            topic3,
            data,
            block_hash,
            block_number,
            tx_hash,
            tx_index,
            log_index,
            removed
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12
            ) ON CONFLICT (address, block_hash, tx_hash) DO NOTHING"#,
            schema = self.schema_name()
        );

        sqlx::query(&sql)
            .bind(log.address.as_slice())
            .bind(log.topics.first().map(|v| v.as_slice()))
            .bind(log.topics.get(1).map(|v| v.as_slice()))
            .bind(log.topics.get(2).map(|v| v.as_slice()))
            .bind(log.topics.get(3).map(|v| v.as_slice()))
            .bind(log.data.0.as_ref())
            .bind(log.block_hash.as_ref().map(|v| v.as_slice()))
            .bind(log.block_number.map(|v| v.to::<i64>()))
            .bind(log.transaction_hash.as_ref().map(|v| v.as_slice()))
            .bind(log.transaction_index.map(|v| v.to::<i64>()))
            .bind(log.log_index.map(|v| v.to::<i64>()))
            .bind(log.removed)
            .execute(&self.inner)
            .await
            .map_err(|e| StorageError::StoreLogFailed {
                addr: log.address,
                err: e.to_string(),
            })?;

        debug!(target: "eventify::core::store::log", address=?log.address, block=?log.block_number, event=?log.topics.first());
        Ok(())
    }

    async fn store_contract(&self, tx: &Contract) -> Result<(), Error> {
        let sql = format!(
            r#"INSERT INTO {schema}.contract (
            tx_hash,
            "from",
            input
            ) VALUES (
                $1, $2, $3
            ) ON CONFLICT DO NOTHING"#,
            schema = self.schema_name()
        );

        sqlx::query(&sql)
            .bind(tx.transaction_hash.as_slice())
            .bind(tx.from.as_slice())
            .bind(tx.input.0.to_vec())
            .execute(&self.inner)
            .await
            .map_err(|e| StorageError::StoreContractFailed {
                hash: tx.transaction_hash,
                err: e.to_string(),
            })?;

        debug!(target: "eventify::core::store::contract", tx_hash=?tx.transaction_hash, tx_from=?tx.from);
        Ok(())
    }

    async fn store_log_transfer(
        &self,
        tx_hash: &FixedBytes<32>,
        from: &FixedBytes<32>,
        to: &FixedBytes<32>,
        value: Bytes,
    ) -> Result<(), Error> {
        let sql = format!(
            r#"INSERT INTO {schema}.log_transfer (
            tx_hash,
            "from",
            "to",
            value )
            VALUES (
                $1, $2, $3, $4
            ) ON CONFLICT DO NOTHING"#,
            schema = self.schema_name()
        );

        sqlx::query(&sql)
            .bind(tx_hash.as_slice())
            .bind(from.as_slice())
            .bind(to.as_slice())
            .bind(value.to_vec())
            .execute(&self.inner)
            .await?;

        debug!(target: "eventify::core::store::eth_transfer", from=?from, to=?to, value=?value);
        Ok(())
    }

    async fn store_log_approval(
        &self,
        tx_hash: &FixedBytes<32>,
        owner: &FixedBytes<32>,
        spender: &FixedBytes<32>,
        value: Bytes,
    ) -> Result<(), Error> {
        let sql = format!(
            r#"INSERT INTO {schema}.log_approval (
            tx_hash,
            owner,
            spender,
            value )
            VALUES (
                $1, $2, $3, $4
            ) ON CONFLICT DO NOTHING"#,
            schema = self.schema_name()
        );

        sqlx::query(&sql)
            .bind(tx_hash.as_slice())
            .bind(owner.as_slice())
            .bind(spender.as_slice())
            .bind(value.to_vec())
            .execute(&self.inner)
            .await?;

        debug!(target: "eventify::core::store::eth_approval", owner=?owner, spender=?spender, value=?value);
        Ok(())
    }

    async fn store_log_approval_for_all(
        &self,
        tx_hash: &FixedBytes<32>,
        owner: &FixedBytes<32>,
        operator: &FixedBytes<32>,
        approved: bool,
    ) -> Result<(), Error> {
        let sql = format!(
            r#"INSERT INTO {schema}.log_approval_for_all (
            tx_hash,
            owner,
            operator,
            approved )
            VALUES (
                $1, $2, $3, $4
            ) ON CONFLICT DO NOTHING"#,
            schema = self.schema_name()
        );

        sqlx::query(&sql)
            .bind(tx_hash.as_slice())
            .bind(owner.as_slice())
            .bind(operator.as_slice())
            .bind(approved)
            .execute(&self.inner)
            .await?;

        debug!(target: "eventify::core::store::eth_approval_for_all", owner=?owner, operator=?operator, approved=?approved);
        Ok(())
    }

    async fn store_log_sent(
        &self,
        tx_hash: &FixedBytes<32>,
        operator: &FixedBytes<32>,
        from: &FixedBytes<32>,
        to: &FixedBytes<32>,
        amount: Bytes,
        data: Bytes,
        operator_data: Bytes,
    ) -> Result<(), Error> {
        let sql = format!(
            r#"INSERT INTO {schema}.log_sent (
            tx_hash,
            operator,
            from,
            to,
            amount,
            data,
            operator_data )
            VALUES (
                $1, $2, $3, $4, $5, $6, $7
            ) ON CONFLICT DO NOTHING"#,
            schema = self.schema_name()
        );

        sqlx::query(&sql)
            .bind(tx_hash.as_slice())
            .bind(operator.as_slice())
            .bind(from.as_slice())
            .bind(to.as_slice())
            .bind(amount.to_vec())
            .bind(data.to_vec())
            .bind(operator_data.to_vec())
            .execute(&self.inner)
            .await?;

        debug!(target: "eventify::core::store::eth_sent", operator=?operator, from=?from, to=?to, amount=?amount);
        Ok(())
    }

    async fn store_log_minted(
        &self,
        tx_hash: &FixedBytes<32>,
        operator: &FixedBytes<32>,
        to: &FixedBytes<32>,
        amount: Bytes,
        data: Bytes,
        operator_data: Bytes,
    ) -> Result<(), Error> {
        let sql = format!(
            r#"INSERT INTO {schema}.log_minted (
            tx_hash,
            operator,
            to,
            amount,
            data,
            operator_data )
            VALUES (
                $1, $2, $3, $4, $5, $6
            ) ON CONFLICT DO NOTHING"#,
            schema = self.schema_name()
        );

        sqlx::query(&sql)
            .bind(tx_hash.as_slice())
            .bind(operator.as_slice())
            .bind(to.as_slice())
            .bind(amount.to_vec())
            .bind(data.to_vec())
            .bind(operator_data.to_vec())
            .execute(&self.inner)
            .await?;

        debug!(target: "eventify::core::store::eth_minted", operator=?operator, to=?to, amount=?amount);
        Ok(())
    }

    async fn store_log_burned(
        &self,
        tx_hash: &FixedBytes<32>,
        operator: &FixedBytes<32>,
        from: &FixedBytes<32>,
        amount: Bytes,
        data: Bytes,
        operator_data: Bytes,
    ) -> Result<(), Error> {
        let sql = format!(
            r#"INSERT INTO {schema}.log_burned (
            tx_hash,
            operator,
            from,
            amount,
            data,
            operator_data )
            VALUES (
                $1, $2, $3, $4, $5, $6
            ) ON CONFLICT DO NOTHING"#,
            schema = self.schema_name()
        );

        sqlx::query(&sql)
            .bind(tx_hash.as_slice())
            .bind(operator.as_slice())
            .bind(from.as_slice())
            .bind(amount.to_vec())
            .bind(data.to_vec())
            .bind(operator_data.to_vec())
            .execute(&self.inner)
            .await?;

        debug!(target: "eventify::core::store::eth_burned", operator=?operator, from=?from, amount=?amount);
        Ok(())
    }

    async fn store_log_authorized_operator(
        &self,
        tx_hash: &FixedBytes<32>,
        operator: &FixedBytes<32>,
        holder: &FixedBytes<32>,
    ) -> Result<(), Error> {
        let sql = format!(
            r#"INSERT INTO {schema}.log_authorized_operator (
            tx_hash,
            operator,
            holder )
            VALUES (
                $1, $2, $3
            ) ON CONFLICT DO NOTHING"#,
            schema = self.schema_name()
        );

        sqlx::query(&sql)
            .bind(tx_hash.as_slice())
            .bind(operator.as_slice())
            .bind(holder.as_slice())
            .execute(&self.inner)
            .await?;

        debug!(target: "eventify::core::store::eth_authorized_operator", operator=?operator, holder=?holder);
        Ok(())
    }

    async fn store_log_revoked_operator(
        &self,
        tx_hash: &FixedBytes<32>,
        operator: &FixedBytes<32>,
        holder: &FixedBytes<32>,
    ) -> Result<(), Error> {
        let sql = format!(
            r#"INSERT INTO {schema}.log_revoked_operator (
            tx_hash,
            operator,
            holder )
            VALUES (
                $1, $2, $3
            ) ON CONFLICT DO NOTHING"#,
            schema = self.schema_name()
        );

        sqlx::query(&sql)
            .bind(tx_hash.as_slice())
            .bind(operator.as_slice())
            .bind(holder.as_slice())
            .execute(&self.inner)
            .await?;

        debug!(target: "eventify::core::store::eth_revoked_operator", operator=?operator, holder=?holder);
        Ok(())
    }

    async fn store_log_transfer_single(
        &self,
        tx_hash: &FixedBytes<32>,
        operator: &FixedBytes<32>,
        from: &FixedBytes<32>,
        to: &FixedBytes<32>,
        id: U64,
        value: Bytes,
    ) -> Result<(), Error> {
        let sql = format!(
            r#"INSERT INTO {schema}.log_transfer_single (
            tx_hash,
            operator,
            from,
            to,
            id,
            value )
            VALUES (
                $1, $2, $3, $4, $5, $6
            ) ON CONFLICT DO NOTHING"#,
            schema = self.schema_name()
        );

        sqlx::query(&sql)
            .bind(tx_hash.as_slice())
            .bind(operator.as_slice())
            .bind(from.as_slice())
            .bind(to.as_slice())
            .bind(id.to::<i64>())
            .bind(value.to_vec())
            .execute(&self.inner)
            .await?;

        debug!(target: "eventify::core::store::eth_transfer_single", operator=?operator, from=?from, to=?to, id=?id, value=?value);
        Ok(())
    }

    async fn store_log_transfer_batch(
        &self,
        tx_hash: &FixedBytes<32>,
        operator: &FixedBytes<32>,
        from: &FixedBytes<32>,
        to: &FixedBytes<32>,
        ids: Vec<U64>,
        values: Vec<Bytes>,
    ) -> Result<(), Error> {
        let sql = format!(
            r#"INSERT INTO {schema}.log_transfer_batch (
            tx_hash,
            operator,
            from,
            to,
            ids,
            values )
            VALUES (
                $1, $2, $3, $4, $5, $6
            ) ON CONFLICT DO NOTHING"#,
            schema = self.schema_name()
        );

        sqlx::query(&sql)
            .bind(tx_hash.as_slice())
            .bind(operator.as_slice())
            .bind(from.as_slice())
            .bind(to.as_slice())
            .bind(ids.iter().map(|v| v.to::<i64>()).collect::<Vec<_>>())
            .bind(values.iter().map(|v| v.to_vec()).collect::<Vec<_>>())
            .execute(&self.inner)
            .await?;

        debug!(target: "eventify::core::store::eth_transfer_batch", operator=?operator, from=?from, to=?to, ids=?ids, values=?values);
        Ok(())
    }

    async fn store_log_uri(
        &self,
        tx_hash: &FixedBytes<32>,
        uri: String,
        id: U64,
    ) -> Result<(), Error> {
        let sql = format!(
            r#"INSERT INTO {schema}.log_uri (
            tx_hash,
            uri,
            id )
            VALUES (
                $1, $2, $3
            ) ON CONFLICT DO NOTHING"#,
            schema = self.schema_name()
        );

        sqlx::query(&sql)
            .bind(tx_hash.as_slice())
            .bind(uri)
            .bind(id.to::<i64>())
            .execute(&self.inner)
            .await?;

        //debug!(target: "eventify::core::store::eth_uri", tx_hash=?tx_hash, uri=?uri, id=?id);
        Ok(())
    }

    async fn store_log_deposit(
        &self,
        tx_hash: &FixedBytes<32>,
        sender: &FixedBytes<32>,
        owner: &FixedBytes<32>,
        assets: U64,
        shares: U64,
    ) -> Result<(), Error> {
        let sql = format!(
            r#"INSERT INTO {schema}.log_deposit (
            tx_hash,
            sender,
            owner,
            assets,
            shares )
            VALUES (
                $1, $2, $3, $4, $5
            ) ON CONFLICT DO NOTHING"#,
            schema = self.schema_name()
        );

        sqlx::query(&sql)
            .bind(tx_hash.as_slice())
            .bind(sender.as_slice())
            .bind(owner.as_slice())
            .bind(assets.to::<i64>())
            .bind(shares.to::<i64>())
            .execute(&self.inner)
            .await?;

        debug!(target: "eventify::core::store::eth_deposit", tx_hash=?tx_hash, sender=?sender, owner=?owner, assets=?assets, shares=?shares);
        Ok(())
    }

    async fn store_log_withdraw(
        &self,
        tx_hash: &FixedBytes<32>,
        sender: &FixedBytes<32>,
        receiver: &FixedBytes<32>,
        owner: &FixedBytes<32>,
        assets: U64,
        shares: U64,
    ) -> Result<(), Error> {
        let sql = format!(
            r#"INSERT INTO {schema}.log_withdraw (
            tx_hash,
            sender,
            receiver,
            owner,
            assets,
            shares )
            VALUES (
                $1, $2, $3, $4, $5, $6
            ) ON CONFLICT DO NOTHING"#,
            schema = self.schema_name()
        );

        sqlx::query(&sql)
            .bind(tx_hash.as_slice())
            .bind(sender.as_slice())
            .bind(receiver.as_slice())
            .bind(owner.as_slice())
            .bind(assets.to::<i64>())
            .bind(shares.to::<i64>())
            .execute(&self.inner)
            .await?;

        debug!(target: "eventify::core::store::eth_withdraw", tx_hash=?tx_hash, owner=?owner, assets=?assets, shares=?shares);
        Ok(())
    }
}
