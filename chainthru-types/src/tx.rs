use async_trait::async_trait;
use ethereum_types::U64;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, Row};
use web3::types::{Bytes, Transaction, H160, H256, H64, U256};

use crate::erc20::{
    Approve, Transfer, TransferFrom, ERC20_APPROVE_SIGNATURE, ERC20_TRANSFER_FROM_SIGNATURE,
    ERC20_TRANSFER_SIGNATURE,
};
use crate::macros::ContractFunction;
use crate::{Insert, Result};

/// Minimum block representation
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct IndexedTransaction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash: Option<H256>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<H160>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<H160>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<Bytes>,
}

impl From<Transaction> for IndexedTransaction {
    fn from(transaction: Transaction) -> Self {
        Self {
            hash: Some(transaction.hash),
            from: transaction.from,
            to: transaction.to,
            input: Some(transaction.input),
        }
    }
}

#[async_trait]
impl Insert for IndexedTransaction {
    async fn insert(&self, dbconn: &PgPool) -> Result<()> {
        if let Some(s) = &self.input {
            if s.0.len() < 4 {
                return Ok(());
            }

            match &s.0[0..4] {
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
        }
        Ok(())
    }
}
