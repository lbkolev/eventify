use web3::types::BlockId;
use web3::Transport;

use crate::{App, Result};
use chainthru_primitives::Insertable;

pub async fn run<T: Transport>(app: App<T>) -> Result<()> {
    let from = app.src_block();
    let to = app.dst_block().await?;

    for target in from..=to {
        let (block, transactions) = app
            .fetch_indexed_data(BlockId::Number(target.into()))
            .await
            .unwrap();

        let db_transaction = app.dbconn().begin().await?;

        match block.insert(app.dbconn()).await {
            Ok(_) => {
                log::info!("Processed block: {:?}", block);
            }
            Err(e) => {
                log::error!("Error processing block: {:?}", e);
            }
        }
        for transaction in transactions {
            match transaction.process(app.dbconn()).await {
                Ok(_) => {
                    log::info!("Processed transaction: {:?}", transaction);
                }
                Err(e) => {
                    log::error!("Error processing transaction: {:?}", e);
                }
            }
        }

        db_transaction.commit().await?;
    }
    Ok(())
}

#[cfg(feature = "parallelism")]
pub async fn run_par() {
    todo!()
}
