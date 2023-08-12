use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use web3::types::{Bytes, Transaction, H160, H256};

use crate::erc20::{
    Approve, Transfer, TransferFrom, ERC20_APPROVE_SIGNATURE, ERC20_TRANSFER_FROM_SIGNATURE,
    ERC20_TRANSFER_SIGNATURE,
};
use crate::{Insertable, Result};

/// Minimum block representation
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct IndexedTransaction {
    pub hash: H256,

    pub from: Option<H160>,

    pub to: Option<H160>,

    pub input: Bytes,
}

impl From<Transaction> for IndexedTransaction {
    fn from(transaction: Transaction) -> Self {
        Self {
            hash: transaction.hash,
            from: transaction.from,
            to: transaction.to,
            input: transaction.input,
        }
    }
}

#[async_trait]
impl Insertable for IndexedTransaction {
    async fn insert(&self, dbconn: &PgPool) -> Result<()> {
        if self.input.0.len() < 4 {
            return Ok(());
        }

        match &self.input.0[0..4] {
            ERC20_APPROVE_SIGNATURE => {
                Approve::from(self.clone()).insert(dbconn).await?;
            }
            ERC20_TRANSFER_FROM_SIGNATURE => {
                TransferFrom::from(self.clone()).insert(dbconn).await?;
            }
            ERC20_TRANSFER_SIGNATURE => {
                Transfer::from(self.clone()).insert(dbconn).await?;
            }

            _ => {}
        }

        Ok(())
    }
}

impl IndexedTransaction {
    pub fn contract_creation(&self) -> bool {
        self.to.is_none()
    }

    pub fn erc20(&self) -> bool {
        self.input.0.len() >= 4
            && (&self.input.0[0..4] == ERC20_APPROVE_SIGNATURE
                || &self.input.0[0..4] == ERC20_TRANSFER_FROM_SIGNATURE
                || &self.input.0[0..4] == ERC20_TRANSFER_SIGNATURE)
    }


}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Contract {
    pub address: H160,
    pub transaction_hash: H256,
    pub from: H160,
    pub input: Bytes,
}

#[async_trait]
impl Insertable for Contract {
    async fn insert(&self, dbconn: &PgPool) -> Result<()> {
        let sql = "INSERT INTO public.contract 
            (contract_addr, transaction_hash, _from, input) 
            VALUES ($1, $2, $3, $4) 
            ON CONFLICT DO NOTHING";

        let tmp = &self.input.0;
        sqlx::query(sql)
            .bind(self.address.as_bytes())
            .bind(self.transaction_hash.as_bytes())
            .bind(self.from.as_bytes())
            .bind(tmp)
            .execute(dbconn)
            .await?;

        Ok(())
    }
}

#[derive(
    derive_builder::Builder, Clone, Debug, Default, serde::Deserialize, serde::Serialize, PartialEq,
)]
#[serde(rename_all = "camelCase")]
pub struct TransactionBoilerplate {
    pub contract_addr: H160,
    pub transaction_hash: H256,
    pub transaction_sender: H160,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_transaction() {
        let json = serde_json::json!({
            "hash": "0x422fb0d5953c0c48cbb42fb58e1c30f5e150441c68374d70ca7d4f191fd56f26",
            "from": "0x407d73d8a49eeb85d32cf465507dd71d507100c1",
            "to": "0x853f43d8a49eeb85d32cf465507dd71d507100c1",
            "input": "0x603880600c6000396000f300603880600c6000396000f3603880600c6000396000f360",
        });

        serde_json::from_value::<IndexedTransaction>(json).unwrap();
    }

    #[test]
    fn serialize_empty_transaction() {
        let json = serde_json::json!({});

        serde_json::from_value::<IndexedTransaction>(json).unwrap();
    }

    #[test]
    fn serialize_contract() {
        let json = serde_json::json!({
            "address": "0x407d73d8a49eeb85d32cf465507dd71d507100c1",
            "transactionHash": "0x422fb0d5953c0c48cbb42fb58e1c30f5e150441c68374d70ca7d4f191fd56f26",
            "from": "0x407d73d8a49eeb85d32cf465507dd71d507100c1",
            "input": "0x603880600c6000396000f300603880600c6000396000f3603880600c6000396000f360",
        });

        serde_json::from_value::<Contract>(json).unwrap();
    }

    #[test]
    fn serialize_transaction_boilerplate() {
        let json = serde_json::json!({
            "contractAddr": "0x407d73d8a49eeb85d32cf465507dd71d507100c1",
            "transactionHash": "0x422fb0d5953c0c48cbb42fb58e1c30f5e150441c68374d70ca7d4f191fd56f26",
            "transactionSender": "0x407d73d8a49eeb85d32cf465507dd71d507100c1",
        });

        serde_json::from_value::<TransactionBoilerplate>(json).unwrap();
    }
}
