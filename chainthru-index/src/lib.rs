pub mod app;
pub mod block;
pub mod transaction;

use app::App;
use web3::types::{BlockId, BlockNumber};
use web3::{types::Transaction, Transport};

use crate::transaction::{erc20::TRANSFER_SIGNATURE, TransactionType};

type Result<T> = std::result::Result<T, crate::Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("SQL error: {0}")]
    Sql(#[from] sqlx::Error),

    #[error("Web3 error: {0}")]
    Web3(#[from] web3::Error),

    #[error("Migration error: {0}")]
    Migrate(#[from] sqlx::migrate::MigrateError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("URL error: {0}")]
    Url(#[from] url::ParseError),
}

pub fn transaction_type(transaction: Transaction) -> transaction::TransactionType {
    if transaction.input.0.starts_with(TRANSFER_SIGNATURE) && transaction.input.0.len() == 68 {
        TransactionType::ERC20
    } else {
        TransactionType::Other
    }
}

pub async fn run<T: Transport>(app: &App<T>) -> Result<()> {
    let from = match app.block_from {
        BlockId::Number(block) => match block {
            BlockNumber::Number(block) => block.as_u64(),
            _ => 0,
        },
        _ => unimplemented!(),
    };

    let to = match app.block_to {
        BlockId::Number(block) => match block {
            BlockNumber::Number(block) => block.as_u64(),
            _ => app.latest_block().await?,
        },
        _ => unimplemented!(),
    };

    for block in from..=to {
        let block = app.fetch_block(BlockId::Number(block.into())).await?;
        app.process_block(block).await;
    }

    Ok(())
}
