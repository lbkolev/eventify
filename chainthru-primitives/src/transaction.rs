use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use sqlx::Row;
use web3::types::{Bytes, Transaction, H160, H256};

use crate::{
    func::{Approve, Transfer, TransferFrom},
    ERC20_APPROVE_SIGNATURE, ERC20_TRANSFER_FROM_SIGNATURE, ERC20_TRANSFER_SIGNATURE,
    ERC721_SAFE_TRANSFER_FROM_SIGNATURE,
};
use crate::{Error, Insertable, Result};

/// There are four types of transactions in the context of the program:
///
/// 1. Contract creation ones
/// 2. Transactions that are considered *special* and have their own tables they're indexed into
/// 3. Transactions that are not considered special but are nonetheless indexed
/// 4. Transactions that are not considered special and are not indexed
///
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
        let sql = "INSERT INTO public.transaction 
            (transaction_hash, _from, _to, input) 
            VALUES ($1, $2, $3, $4) 
            ON CONFLICT DO NOTHING";

        let tmp = &self.input.0;
        sqlx::query(sql)
            .bind(self.hash.as_bytes())
            .bind(self.from.as_ref().map(|x| x.as_bytes()))
            .bind(self.to.as_ref().map(|x| x.as_bytes()))
            .bind(tmp)
            .execute(dbconn)
            .await?;

        Ok(())
    }
}

impl IndexedTransaction {
    /// Determines if the transaction is a contract creation one.
    pub fn contract_creation(&self) -> bool {
        self.to.is_none()
    }

    /// Determines if the transaction is considered special.
    ///
    /// Special transactions are the ones that are indexed into their own tables.
    fn special(&self) -> bool {
        self.input.0.len() >= 4
            && (&self.input.0[0..4] == ERC20_APPROVE_SIGNATURE
                || &self.input.0[0..4] == ERC20_TRANSFER_FROM_SIGNATURE
                || &self.input.0[0..4] == ERC20_TRANSFER_SIGNATURE
                || &self.input.0[0..4] == ERC721_SAFE_TRANSFER_FROM_SIGNATURE)
    }

    /// Determines if the transaction is indexed at all.
    async fn tracked(&self, conn: &PgPool) -> Result<bool> {
        if self.input.0.len() < 4 {
            return Ok(false);
        }

        let sql = "SELECT EXISTS(SELECT 1 FROM public.function_signature WHERE hex_sig = $1)";
        match sqlx::query(sql)
            .bind(&self.input.0[0..4])
            .fetch_one(conn)
            .await
        {
            Ok(row) => Ok(row.get(0)),
            Err(e) => Err(Error::from(e)),
        }
    }

    /// Processes the transaction.
    ///
    /// If the transaction is considered special, it's indexed into its own table.
    /// If the transaction is not considered special, but we've got a function signature that matches the transaction's input, it is indexed into the `transaction` table.
    pub async fn process(&self, conn: &PgPool) -> Result<()> {
        if self.tracked(conn).await? && self.special() {
            match &self.input.0[0..4] {
                ERC20_APPROVE_SIGNATURE => Approve::try_from(self.clone())?.insert(conn).await,
                ERC20_TRANSFER_FROM_SIGNATURE => {
                    TransferFrom::try_from(self.clone())?.insert(conn).await
                }
                ERC20_TRANSFER_SIGNATURE => Transfer::try_from(self.clone())?.insert(conn).await,
                ERC721_SAFE_TRANSFER_FROM_SIGNATURE => {
                    unimplemented!("ERC721 safe transfer from")
                }
                _ => Err(Error::InvalidTransactionFunctionSignature(
                    String::from_utf8_lossy(&self.input.0[0..4]).to_string(),
                )),
            }
        } else if self.tracked(conn).await? {
            self.insert(conn).await
        } else {
            Ok(())
        }
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
    fn serialize_half_empty_transaction() {
        let json = serde_json::json!({
            "hash": "0x422fb0d5953c0c48cbb42fb58e1c30f5e150441c68374d70ca7d4f191fd56f26",
            "from": None::<H160>,
            "to": None::<H160>,
            "input": "0x603880600c6000396000f300603880600c6000396000f3603880600c6000396000f360",
        });

        serde_json::from_value::<IndexedTransaction>(json).unwrap();
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
