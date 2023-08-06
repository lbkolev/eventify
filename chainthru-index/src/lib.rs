pub mod app;
pub mod error;
pub mod tx;

use web3::types::{BlockId, Transaction};
use web3::Transport;

use crate::tx::{ERC20_APPROVE_SIGNATURE, ERC20_TRANSFER_FROM_SIGNATURE, ERC20_TRANSFER_SIGNATURE};
use chainthru_types::{erc20, IndexedBlock, Insert};

/// The result type used throughout the indexer
type Result<T> = std::result::Result<T, error::Error>;

pub enum Contract {
    ERC20,
    Other,
}

pub async fn run<T: Transport>(app: app::App<T>) -> Result<()> {
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
                match IndexedBlock::from(block.clone()).insert(&db_handler).await {
                    Ok(_) => {
                        log::debug!("Indexed block");
                    }
                    Err(e) => {
                        log::error!("Error indexing block : {}", e);
                        db_transaction.rollback().await?;
                        continue;
                    }
                }

                for tx in block.transactions {
                    // The type of transaction is determined by the initial bytes & the length of the input data
                    if tx.input.0.starts_with(ERC20_TRANSFER_SIGNATURE) && tx.input.0.len() == 68 {
                        log::debug!("ERC20 transfer detected: {:#?}", tx);
                        let tf = erc20::Transfer::from(tx);

                        tf.insert(&db_handler).await?;
                    } else if tx.input.0.starts_with(ERC20_TRANSFER_FROM_SIGNATURE) {
                        log::debug!("ERC20 transferFrom detected: {:#?}", tx);

                        let tf = erc20::TransferFrom::from(tx);
                        tf.insert(&db_handler).await?;
                    } else if tx.input.0.starts_with(ERC20_APPROVE_SIGNATURE) {
                        log::debug!("ERC20 approve detected: {:#?}", tx);

                        let tf = erc20::Approve::from(tx);
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
