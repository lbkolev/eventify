use alloy_primitives::BlockNumber;
use ethers_providers::JsonRpcClient;

use crate::{App, Result};
use chainthru_primitives::{Auth, Criterias, Storage};

pub async fn run<T: JsonRpcClient + Clone, U: Storage + Auth + Clone>(
    app: App<T, U>,
    criterias: Option<Criterias>,
) -> Result<()> {
    let from = app.src_block();
    let to = app.dst_block();

    for target in from..=to {
        log::info!("Fetching block: {}", target);
        let log = app.fetch_logs(criterias.as_ref().unwrap()).await?;
        log::info!("Logs: {:?}", log);

        let (block, transactions) = match app.fetch_indexed_data(target).await {
            Ok((block, transactions)) => (block, transactions),
            Err(_) => {
                if target >= app.get_latest_block().await? {
                    log::info!("Reached latest block: {:?}", target);
                    break;
                }

                continue;
            }
        };

        app.storage_conn()?.insert_block(&block).await?;
        for tx in transactions {
            if tx.contract_creation() {
                app.storage_conn()?
                    .insert_contract(&tx.clone().into())
                    .await?;
            }
            app.storage_conn()?.insert_transaction(&tx).await?;
        }
    }
    Ok(())
}

// TODO
#[allow(unused)]
pub async fn run_bare<T: JsonRpcClient + Clone + 'static, U: Storage + Auth + Clone>(
    app: App<T, U>,
    from: BlockNumber,
    to: BlockNumber,
) -> Result<()> {
    for target in from..=to {
        log::info!("Fetching block: {}", target);
        let (block, transactions) = match app.fetch_indexed_data(target).await {
            Ok((block, transactions)) => (block, transactions),
            Err(_) => {
                log::warn!("Error fetching block: {:?}", target);

                if target >= app.get_latest_block().await.unwrap() {
                    log::info!("Reached latest block: {:?}", target);
                    break;
                }

                continue;
            }
        };

        app.storage_conn()?.insert_block(&block).await?;
        for tx in transactions {
            if tx.contract_creation() {
                app.storage_conn()?
                    .insert_contract(&tx.clone().into())
                    .await?;
            }
            app.storage_conn()?.insert_transaction(&tx).await?;
        }
    }

    Ok(())
}

// TODO
//#[cfg(feature = "parallelism")]
#[allow(unused)]
pub async fn run_par<T: JsonRpcClient + Clone + 'static, U: Storage + Auth + Clone>(
    app: App<T, U>,
    size: u16,
) -> Result<()> {
    let block_range = app.dst_block() - app.src_block() + 1;
    let segment_size = block_range / size as u64;

    let mut handles = vec![];

    let mut tmp_src_block = app.src_block();
    let mut tmp_dst_block = app.dst_block() + segment_size;
    for i in 0..size {
        let app = app.clone();
        let from = tmp_src_block;
        let to = tmp_dst_block;

        tmp_src_block = tmp_dst_block + 1;
        tmp_dst_block += segment_size;

        handles.push(tokio::spawn(run_bare(app, from, to)));
    }

    Ok(())
}
