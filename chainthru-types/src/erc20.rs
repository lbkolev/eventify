use async_trait::async_trait;
use ethereum_types::{H160, H256, U256};

use crate::{contract_func, tx::IndexedTransaction, Insertable, Result, TXBoilerplate};

/// The signature of the ERC20 approve method
pub const ERC20_APPROVE_SIGNATURE: &[u8] = &[0x09, 0xb6, 0x7f, 0x8e];

/// The signature of the ERC20 transfer method
pub const ERC20_TRANSFER_SIGNATURE: &[u8] = &[0xa9, 0x05, 0x9c, 0xbb];

/// The signature of the ERC20 transferFrom method
pub const ERC20_TRANSFER_FROM_SIGNATURE: &[u8] = &[0x23, 0xb8, 0x72, 0xdd];

contract_func!(
    contract=erc20,
    Transfer[
        _to: H160,
        _value: U256
    ]
);

contract_func!(
    contract=erc20,
    TransferFrom[
        _from: H160,
        _to: H160,
        _value: U256
    ]
);

contract_func!(
    contract=erc20,
    Approve[
        _spender: H160,
        _value: U256
    ]
);

impl From<IndexedTransaction> for Transfer {
    fn from(transaction: IndexedTransaction) -> Self {
        let input = transaction.input.expect("Empty transaction input");
        Self {
            boilerplate: TXBoilerplate {
                contract_addr: transaction.to.unwrap_or(H160::default()),
                transaction_hash: transaction.hash.unwrap_or(H256::default()),
                transaction_sender: transaction.from.unwrap_or(H160::default()),
            },

            _to: H160::from_slice(&input.0[16..36]),
            _value: U256::from(&input.0[36..68]),
        }
    }
}

#[async_trait]
impl Insertable for Transfer {
    async fn insert(&self, db_conn: &sqlx::PgPool) -> Result<()> {
        let sql = "
            INSERT INTO erc20.transfer (contract_addr, transaction_hash, transaction_sender, _to, _value)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT DO NOTHING
            ";

        let mut value_slice = [0u8; 32];
        self._value.to_big_endian(&mut value_slice);

        sqlx::query(sql)
            .bind(self.boilerplate.contract_addr.as_bytes())
            .bind(self.boilerplate.transaction_hash.as_bytes())
            .bind(self.boilerplate.transaction_sender.as_bytes())
            .bind(self._to.as_bytes())
            .bind(value_slice)
            .execute(db_conn)
            .await?;

        Ok(())
    }
}

impl From<IndexedTransaction> for TransferFrom {
    fn from(transaction: IndexedTransaction) -> Self {
        let input = transaction.input.expect("Empty transaction input");
        Self {
            boilerplate: TXBoilerplate {
                contract_addr: transaction.to.unwrap_or(H160::default()),
                transaction_hash: transaction.hash.unwrap_or(H256::default()),
                transaction_sender: transaction.from.unwrap_or(H160::default()),
            },

            _from: H160::from_slice(&input.0[16..36]),
            _to: H160::from_slice(&input.0[36..56]),
            _value: U256::from(&input.0[56..78]),
        }
    }
}

#[async_trait]
impl Insertable for TransferFrom {
    async fn insert(&self, db_conn: &sqlx::PgPool) -> Result<()> {
        let sql = "
            INSERT INTO erc20.transfer_from (contract_addr, transaction_hash, transaction_sender, _from, _to, _value)
            VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT DO NOTHING
            ";

        let mut value_slice = [0u8; 32];
        self._value.to_big_endian(&mut value_slice);

        sqlx::query(sql)
            .bind(self.boilerplate.contract_addr.as_bytes())
            .bind(self.boilerplate.transaction_hash.as_bytes())
            .bind(self.boilerplate.transaction_sender.as_bytes())
            .bind(self._from.as_bytes())
            .bind(self._to.as_bytes())
            .bind(value_slice)
            .execute(db_conn)
            .await?;

        Ok(())
    }
}

impl From<IndexedTransaction> for Approve {
    fn from(value: IndexedTransaction) -> Self {
        let input = value.input.expect("Empty transaction input");
        Self {
            boilerplate: TXBoilerplate {
                contract_addr: value.to.unwrap_or(H160::default()),
                transaction_hash: value.hash.unwrap_or(H256::default()),
                transaction_sender: value.from.unwrap_or(H160::default()),
            },
            _spender: H160::from_slice(&input.0[16..36]),
            _value: U256::from(&input.0[36..68]),
        }
    }
}

#[async_trait]
impl Insertable for Approve {
    async fn insert(&self, db_conn: &sqlx::PgPool) -> Result<()> {
        let sql = "
            INSERT INTO erc20.approve (contract_addr, transaction_hash, transaction_sender, _spender, _value)
            VALUES ($1, $2, $3, $4, $5::numeric)
            ON CONFLICT DO NOTHING
            ";

        let mut value_slice = [0u8; 32];
        self._value.to_big_endian(&mut value_slice);

        sqlx::query(sql)
            .bind(self.boilerplate.contract_addr.as_bytes())
            .bind(self.boilerplate.transaction_hash.as_bytes())
            .bind(self.boilerplate.transaction_sender.as_bytes())
            .bind(self._spender.as_bytes())
            .bind(value_slice)
            .execute(db_conn)
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_erc20_transfer() {
        let json = serde_json::json!({
            "contractAddr": "0x407d73d8a49eeb85d32cf465507dd71d507100c1",
            "transactionHash": "0x422fb0d5953c0c48cbb42fb58e1c30f5e150441c68374d70ca7d4f191fd56f26",
            "transactionSender": "0x407d73d8a49eeb85d32cf465507dd71d507100c1",
            "_to": "0x853f43d8a49eeb85d32cf465507dd71d507100c1",
            "_value": "0x00000000"
        });

        serde_json::from_value::<Transfer>(json).unwrap();
    }

    #[test]
    fn serialize_erc20_transfer_from() {
        let json = serde_json::json!({
            "contractAddr": "0x407d73d8a49eeb85d32cf465507dd71d507100c1",
            "transactionHash": "0x422fb0d5953c0c48cbb42fb58e1c30f5e150441c68374d70ca7d4f191fd56f26",
            "transactionSender": "0x407d73d8a49eeb85d32cf465507dd71d507100c1",
            "_from": "0x853f43d8a49eeb85d32cf465507dd71d507100c1",
            "_to": "0x853f43d8a49eeb85d32cf465507dd71d507100c1",
            "_value": "0x00000000"
        });

        serde_json::from_value::<TransferFrom>(json).unwrap();
    }

    #[test]
    fn serialize_erc20_approve() {
        let json = serde_json::json!({
            "contractAddr": "0x407d73d8a49eeb85d32cf465507dd71d507100c1",
            "transactionHash": "0x422fb0d5953c0c48cbb42fb58e1c30f5e150441c68374d70ca7d4f191fd56f26",
            "transactionSender": "0x407d73d8a49eeb85d32cf465507dd71d507100c1",
            "_spender": "0x853f43d8a49eeb85d32cf465507dd71d507100c1",
            "_value": "0x00000000"
        });

        serde_json::from_value::<Approve>(json).unwrap();
    }
}
