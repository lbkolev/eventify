use ethereum_types::{H256, H64, U256, U64};
use web3::types::{Block, Transaction};

pub async fn insert_block(
    block: &Block<Transaction>,
    db_conn: &sqlx::PgPool,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO block
        (hash, parent_hash, uncles_hash, author, state_root, transactions_root, receipts_root, number, gas_used, gas_limit, base_fee_per_gas, timestamp, difficulty, total_difficulty, transactions, size, nonce)
        VALUES
        ($1, $2, $3, $4, $5, $6, $7, $8, $9::numeric, $10::numeric, $11::numeric, $12::bigint, $13::numeric, $14::numeric, $15, $16, $17)",
    )
        .bind(block.hash.unwrap_or(H256::zero()).as_bytes())
        .bind(block.parent_hash.as_bytes())
        .bind(block.uncles_hash.as_bytes())
        .bind(block.author.as_bytes())
        .bind(block.state_root.as_bytes())
        .bind(block.transactions_root.as_bytes())
        .bind(block.receipts_root.as_bytes())
        .bind(block.number.unwrap_or(U64::zero()).as_u64() as i64)
        .bind(block.gas_used.to_string())
        .bind(block.gas_limit.to_string())
        .bind(block.base_fee_per_gas.unwrap_or(U256::zero()).to_string())
        .bind(block.timestamp.to_string())
        .bind(block.difficulty.to_string())
        .bind(block.total_difficulty.unwrap_or(U256::zero()).to_string())
        .bind(block.transactions.len() as i64)
        .bind(block.size.unwrap_or(U256::zero()).as_u64() as i64)
        .bind(block.nonce.unwrap_or(H64::zero()).as_bytes())
        .execute(db_conn).await?;

    Ok(())
}
