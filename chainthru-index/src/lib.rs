pub mod app;
pub mod block;
pub mod tx;

use app::App;
use chainthru_types::Insert;
use web3::types::{BlockId, BlockNumber, Transaction};
use web3::Transport;

use crate::tx::{ERC20_APPROVE_SIGNATURE, ERC20_TRANSFER_FROM_SIGNATURE, ERC20_TRANSFER_SIGNATURE};
use chainthru_types::{
    erc20::{Approve, Transfer, TransferFrom},
    IndexedBlock,
};

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

pub enum Contract {
    ERC20,
    Other,
}

pub async fn run<T: Transport>(app: App<T>) -> Result<()> {
    let from = app.src_block().await;
    let to = app.dst_block().await?;
    let db_handler = app.dbconn().await?;

    for target in from..=to {
        let block = app.fetch_block(BlockId::Number(target.into())).await?;
        log::debug!("Processing block {:#?}", block);

        if target % 250 == 0 {
            log::info!("Processed 250 blocks [{}, {})", target - 250, target);
        }

        match block {
            Some(block) => {
                let db_transaction = db_handler.begin().await?;
                match IndexedBlock::from(block).insert(&db_handler).await {
                    Ok(_) => {
                        log::debug!("Indexed block");
                    }
                    Err(e) => {
                        log::error!("Error indexing block : {}", e);
                        db_transaction.rollback().await?;
                        continue;
                    }
                }
                //block::insert(&block, &db_handler).await?;

                /*
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
                */
                db_transaction.commit().await?;
            }

            None => {
                log::warn!("Block {:#?} not found", block);
            }
        }
    }

    Ok(())
}

// #[cfg(feature = "parallelism")]
pub async fn run_par() -> () {
    unimplemented!()
}

fn tx_type(tx: Transaction) -> Contract {
    match &tx.input.0[0..4] {
        ERC20_TRANSFER_SIGNATURE | ERC20_TRANSFER_FROM_SIGNATURE | ERC20_APPROVE_SIGNATURE => {
            Contract::ERC20
        }
        _ => Contract::Other,
    }
}
