use web3::{types::BlockId, Transport};

use crate::{App, Result};
use chainthru_primitives::{Auth, Storage};

pub async fn run<T: Transport, U: Storage + Auth>(app: App<T, U>) -> Result<()> {
    let from = app.src_block();
    let to = app.dst_block().await?;

    for target in from..=to {
        let (block, transactions) = app
            .fetch_indexed_data(BlockId::Number(target.into()))
            .await
            .unwrap();

        match app.storage_conn().insert_block(&block).await {
            Ok(_) => {
                log::info!("Processed block: {:?}", block);
            }
            Err(e) => {
                log::error!("Error processing block: {:?}", e);
            }
        }
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
    }
    Ok(())
}

#[cfg(feature = "parallelism")]
pub async fn run_par() {
    todo!()
}
