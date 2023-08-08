pub mod app;
pub mod error;

use web3::types::BlockId;
use web3::Transport;

use chainthru_types::Insertable;

/// The result type used throughout the indexer
type Result<T> = std::result::Result<T, error::Error>;

pub async fn run<T: Transport>(app: app::App<T>) -> Result<()> {
    let from = app.src_block().await;
    let to = app.dst_block().await?;
    let db_handler = app.dbconn().await?;

    for target in from..=to {
        let (block, transactions) = app
            .fetch_indexed_data(BlockId::Number(target.into()))
            .await
            .unwrap();

        let db_transaction = db_handler.begin().await?;
        block.insert(&db_handler).await;
        for transaction in transactions {
            transaction.insert(&db_handler).await;
        }
        db_transaction.commit().await?;
    }

    Ok(())
}

#[cfg(feature = "parallelism")]
pub async fn run_par() {
    todo!()
}
