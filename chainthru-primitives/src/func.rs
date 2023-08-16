use async_trait::async_trait;
use ethereum_types::{H160, U256};

use crate::{
    contract_func, error::Error, transaction::IndexedTransaction, Insertable, Result,
    TransactionBoilerplate,
};

contract_func!(
    Transfer[
        _to: H160,
        _value: U256
    ]
);

contract_func!(
    TransferFrom[
        _from: H160,
        _to: H160,
        _value: U256
    ]
);

contract_func!(
    Approve[
        _spender: H160,
        _value: U256
    ]
);

impl TryFrom<IndexedTransaction> for Transfer {
    type Error = Error;

    fn try_from(transaction: IndexedTransaction) -> Result<Self> {
        let input = transaction.input;

        Ok(Self {
            boilerplate: TransactionBoilerplate {
                contract_addr: transaction.to.expect("Contract address is None"),
                transaction_hash: transaction.hash,
                transaction_sender: transaction.from.expect("Transaction sender is None"),
            },

            _to: H160::from_slice(&input.0[16..36]),
            _value: U256::from(&input.0[36..68]),
        })
    }
}

#[async_trait]
impl Insertable for Transfer {
    async fn insert(&self, db_conn: &sqlx::PgPool) -> Result<()> {
        let sql = "
            INSERT INTO contract_fn.transfer (contract_addr, transaction_hash, transaction_sender, _to, _value)
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

impl TryFrom<IndexedTransaction> for TransferFrom {
    type Error = Error;

    fn try_from(transaction: IndexedTransaction) -> Result<Self> {
        let input = transaction.input;

        Ok(Self {
            boilerplate: TransactionBoilerplate {
                contract_addr: transaction.to.expect("Contract address is None"),
                transaction_hash: transaction.hash,
                transaction_sender: transaction.from.expect("Transaction sender is None"),
            },

            _from: H160::from_slice(&input.0[16..36]),
            _to: H160::from_slice(&input.0[36..56]),
            _value: U256::from(&input.0[56..78]),
        })
    }
}

#[async_trait]
impl Insertable for TransferFrom {
    async fn insert(&self, db_conn: &sqlx::PgPool) -> Result<()> {
        let sql = "
            INSERT INTO contract_fn.transfer_from (contract_addr, transaction_hash, transaction_sender, _from, _to, _value)
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

impl TryFrom<IndexedTransaction> for Approve {
    type Error = Error;

    fn try_from(value: IndexedTransaction) -> Result<Self> {
        let input = value.input;

        Ok(Self {
            boilerplate: TransactionBoilerplate {
                contract_addr: value.to.expect("Contract address is None"),
                transaction_hash: value.hash,
                transaction_sender: value.from.expect("Transaction sender is None"),
            },
            _spender: H160::from_slice(&input.0[16..36]),
            _value: U256::from(&input.0[36..68]),
        })
    }
}

#[async_trait]
impl Insertable for Approve {
    async fn insert(&self, db_conn: &sqlx::PgPool) -> Result<()> {
        let sql = "
            INSERT INTO contract_fn.approve (contract_addr, transaction_hash, transaction_sender, _spender, _value)
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
