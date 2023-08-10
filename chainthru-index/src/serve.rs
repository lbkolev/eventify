use web3::types::BlockId;
use web3::Transport;

use crate::{App, Result};
use chainthru_types::Insertable;

pub async fn run<T: Transport>(app: App<T>) -> Result<()> {
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
            if transaction.to.is_none() {
                let a = app
                    .fetch_transaction_receipt(transaction.hash.unwrap())
                    .await;

                if let Some(receipt) = a {
                    let contract = chainthru_types::tx::Contract {
                        address: receipt.contract_address.unwrap(),
                        transaction_hash: receipt.transaction_hash,
                        from: transaction.from.unwrap(),
                        input: transaction.input.clone().unwrap(),
                    };
                    contract.insert(&db_handler).await;
                }
            } else {
                transaction.insert(&db_handler).await;
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
