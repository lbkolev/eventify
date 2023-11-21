use web3::Transport;

use crate::{App, Result};
use chainthru_primitives::{Auth, Storage};

pub async fn run<T: Transport, U: Storage + Auth>(app: App<T, U>) -> Result<()> {
    let from = app.src_block();
    let to = app.dst_block();

    for target in from..=to {
        println!("Fetching block: {}", target);
        let (block, transactions) = match app.fetch_indexed_data(target).await {
            Ok((block, transactions)) => (block, transactions),
            Err(_) => {
                log::warn!("Error fetching block: {:?}", target);

                if target >= app.latest_block().await? {
                    log::info!("Reached latest block: {:?}", target);
                    break;
                }

                continue;
            }
        };

        match app.storage_conn().insert_block(&block).await {
            Ok(_) => {
                log::info!("Processed block: {:?}", block);
            }
            Err(e) => {
                log::error!("Error processing block: {:?}", e);
            }
        }

        for tx in transactions {
            match app.storage_conn().insert_transaction(&tx).await {
                Ok(_) => {
                    log::info!("Processed transaction: {:?}", tx);
                }
                Err(e) => {
                    log::error!("Error processing transaction: {:?}", e);
                }
            }
        }

        /*
        for transaction in transactions {
            match transaction.process(app.storage_conn()).await {
                Ok(_) => {
                    log::info!("Processed transaction: {:?}", transaction);
                }
                Err(e) => {
                    log::error!("Error processing transaction: {:?}", e);
                }
            }
        }
        */
    }
    Ok(())
}

#[cfg(feature = "parallelism")]
pub async fn run_par() {
    todo!()
}
