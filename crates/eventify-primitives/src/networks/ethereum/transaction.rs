use alloy_primitives::{Address, Bytes, B256, U256, U64};
use eyre::Result;
use redis::Commands;
use serde::{Deserialize, Serialize};
use sqlx::{Error as SqlError, FromRow};
use utoipa::ToSchema;

use crate::{
    networks::ResourceKind,
    traits::{Emit, Insert, Transaction},
    EmitError,
};

#[derive(Clone, Debug, Default, Hash, Deserialize, Serialize, PartialEq, Eq, FromRow, ToSchema)]
pub struct TransactionResponse {
    pub transactions: Vec<EthTransaction>,
}

#[derive(Clone, Debug, Default, Hash, Deserialize, Serialize, PartialEq, Eq, FromRow, ToSchema)]
pub struct EthTransaction {
    #[serde(rename = "blockHash")]
    pub block_hash: Option<B256>,
    #[serde(rename = "blockNumber")]
    pub block_number: Option<U64>,
    pub from: Address,
    pub gas: U256,
    #[serde(rename = "gasPrice")]
    pub gas_price: U256,
    pub hash: B256,
    pub input: Bytes,
    pub nonce: U256,
    pub to: Option<Address>,
    #[serde(rename = "transactionIndex")]
    pub transaction_index: Option<U64>,
    pub value: U256,
    pub v: U256,
    pub r: U256,
    pub s: U256,
}

impl EthTransaction {
    pub fn contract_creation(&self) -> bool {
        self.to.is_none()
    }
}

impl Insert for EthTransaction {
    async fn insert(
        &self,
        pool: &sqlx::PgPool,
        schema: &str,
        _: &Option<alloy_primitives::B256>,
    ) -> Result<(), SqlError> {
        let from = self.from.as_slice();
        let gas = self.gas.as_le_slice();
        let gas_price = self.gas_price.as_le_slice();
        let hash = self.hash.as_slice();
        let input = self.input.0.as_ref();
        let nonce = self.nonce.as_le_slice();
        let to = self.to.as_ref().map(|v| v.as_slice());
        let value = self.value.as_le_slice();
        let v = self.v.as_le_slice();
        let r = self.r.as_le_slice();
        let s = self.s.as_le_slice();
        let block_hash = self.block_hash.as_ref().map(|v| v.as_slice());
        let block_number = self.block_number.map(|v| v.to::<i64>());
        let tx_index = self.transaction_index.map(|v| v.to::<i64>());

        let sql = format!(
            r#"INSERT INTO {schema}.transaction (
            "from",
            gas,
            gas_price,
            hash,
            input,
            nonce,
            "to",
            value,
            v,
            r,
            s,
            block_hash,
            block_number,
            transaction_index
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)"#,
        );

        sqlx::query(&sql)
            .bind(from)
            .bind(gas)
            .bind(gas_price)
            .bind(hash)
            .bind(input)
            .bind(nonce)
            .bind(to)
            .bind(value)
            .bind(v)
            .bind(r)
            .bind(s)
            .bind(block_hash)
            .bind(block_number)
            .bind(tx_index)
            .execute(pool)
            .await?;

        Ok(())
    }
}

impl Emit for EthTransaction {
    async fn emit(
        &self,
        queue: &redis::Client,
        network: &crate::networks::NetworkKind,
    ) -> Result<(), EmitError> {
        let mut con = queue.get_connection()?;

        let channel = format!("{}:{}", network, ResourceKind::Transaction);
        con.lpush(channel, serde_json::to_string(self)?)?;

        Ok(())
    }
}

impl Transaction for EthTransaction {
    fn block_hash(&self) -> Option<alloy_primitives::B256> {
        self.block_hash
    }

    fn hash(&self) -> alloy_primitives::B256 {
        self.hash
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_tx() {
        let json = serde_json::json!({
            "blockHash":"0x1d59ff54b1eb26b013ce3cb5fc9dab3705b415a67127a003c3e61eb445bb8df2",
            "blockNumber":"0x5daf3b",
            "from":"0xa7d9ddbe1f17865597fbd27ec712455208b6b76d",
            "gas":"0xc350",
            "gasPrice":"0x4a817c800",
            "hash":"0x88df016429689c079f3b2f6ad39fa052532c56795b733da78a91ebe6a713944b",
            "input":"0x68656c6c6f21",
            "nonce":"0x15",
            "to":"0xf02c1c8e6114b1dbe8937a39260b5b0a374432bb",
            "transactionIndex":"0x41",
            "value":"0xf3dbb76162000",
            "v":"0x25",
            "r":"0x1b5e176d927f8e9ab405058b2d2457392da3e20f328b16ddabcebc33eaac5fea",
            "s":"0x4ba69724e8f69de52f0125ad8b3c5c2cef33019bac3249e2c0a2192766d1721c"
        });

        serde_json::from_value::<EthTransaction>(json).unwrap();
    }

    #[test]
    fn deserialize_empty_tx() {
        let json = serde_json::json!({});

        assert!(serde_json::from_value::<EthTransaction>(json).is_err());
    }

    #[test]
    fn test_is_contract_creation() {
        let tx = EthTransaction {
            to: None,
            ..Default::default() // Using other default values
        };

        assert!(tx.contract_creation());
    }
}
