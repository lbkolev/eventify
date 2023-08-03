pub mod app;
pub mod block;
pub mod tx;

use app::App;
use web3::types::{BlockId, BlockNumber};
use web3::Transport;

use crate::tx::{ERC20_APPROVE_SIGNATURE, ERC20_TRANSFER_FROM_SIGNATURE, ERC20_TRANSFER_SIGNATURE};
use chainthru_types::erc20::{Approve, Transfer, TransferFrom};

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

pub async fn run<T: Transport>(app: App<T>) -> Result<()> {
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

    for target in from..=to {
        let block = app.fetch_block(BlockId::Number(target.into())).await?;
        log::debug!("Processing block {:#?}", block);

        if target % 250 == 0 {
            log::info!("Processed 250 blocks [{}, {})", target - 250, target);
        }

        let db_handler = app.dbconn().await?;
        match block {
            Some(block) => {
                let db_transaction = db_handler.begin().await?;
                block::insert(&block, &db_handler).await?;

                for tx in block.transactions {
                    // The type of transaction is determined by the initial bytes & the length of the input data
                    if tx.input.0.starts_with(ERC20_TRANSFER_SIGNATURE) && tx.input.0.len() == 68 {
                        log::debug!("ERC20 transfer detected: {:#?}", tx);
                        let tf = Transfer::from(tx);

                        tf.insert(&db_handler).await?;
                    } else if tx.input.0.starts_with(ERC20_TRANSFER_FROM_SIGNATURE) {
                        log::debug!("ERC20 transferFrom detected: {:#?}", tx);

                        let tf = TransferFrom::from(tx);
                        tf.insert(&db_handler).await?;
                    } else if tx.input.0.starts_with(ERC20_APPROVE_SIGNATURE) {
                        log::debug!("ERC20 approve detected: {:#?}", tx);

                        let tf = Approve::from(tx);
                        tf.insert(&db_handler).await?;
                    } else {
                        log::debug!("Unknown transaction: {:#?}", tx);
                    }
                }

                db_transaction.commit().await?;
            }

            None => {
                log::warn!("Block {:#?} not found", block);
            }
        }
    }

    Ok(())
}
