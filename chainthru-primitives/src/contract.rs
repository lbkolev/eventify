use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use web3::types::{Bytes, H160, H256};

use crate::{Insertable, Result};

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

#[cfg(test)]
mod tests {
    use super::*;

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
}
