pub mod block;
pub mod transaction;

use ethereum_types::{H160, U256};
use web3::types::BlockId;

use crate::block::insert_block;
use transaction::erc20::{self, TRANSFER_SIGNATURE};

pub async fn run(settings: Settings) -> std::result::Result<(), crate::Error> {
    let db_conn = sqlx::PgPool::connect(&settings.database_url).await?;
    sqlx::migrate!().run(&db_conn).await?;

    // Connect to the Ethereum node
    let web3 = web3::Web3::new(web3::transports::Http::new(&settings.node_url)?);

    let begin = settings.from_block;
    let end = match settings.to_block {
        Some(block) => block,
        None => web3.eth().block_number().await?.as_u64(),
    };

    for block in begin..=end {
        // Retrieve the block with transactions
        let block_with_txs = web3
            .eth()
            .block_with_txs(BlockId::Number(block.into()))
            .await?;

        if let Some(block) = block_with_txs {
            insert_block(&block, &db_conn).await?;

            for tx in block.transactions {
                log::info!("{:?}", tx);
                if tx.input.0.starts_with(TRANSFER_SIGNATURE) && tx.input.0.len() == 68 {
                    let transfer = erc20::Method::Transfer(erc20::transfer::Transfer {
                        hash: tx.hash,
                        from: tx.from.unwrap(),
                        to: H160::from_slice(&tx.input.0[16..36]),
                        value: U256::from(&tx.input.0[36..68]),
                    });

                    log::info!("{:?}", transfer);
                    if let Some(to) = tx.to {
                        let erc20 = erc20::ERC20::new(to, transfer);
                        erc20.insert(&db_conn).await?;
                    }
                }
            }
        }
    }

    Ok(())
}

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
}

#[derive(Debug, Clone)]
pub struct Settings {
    pub database_url: String,
    pub node_url: String,
    pub from_block: u64,
    pub to_block: Option<u64>,
}
