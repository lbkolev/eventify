use ethers_providers::JsonRpcClient;
use tokio::task::JoinHandle;

use crate::{App, Result};
use chainthru_primitives::{Auth, Criterias, Storage};

pub async fn run<T: JsonRpcClient + Clone, U: Storage + Auth + Clone>(
    app: App<T, U>,
    criterias: Option<Criterias>,
) -> Result<()> {
    let from = app.src_block();
    let to = app.dst_block();

    for target in from..=to {
        if let Some(crits) = criterias.as_ref() {
            let logs = app.fetch_logs(crits, target).await?;
            log::info!("{:?}", logs);

            for log in logs {
                println!("{:?}", log.clone());
                app.storage_conn()?.insert_log(&log.into()).await?;
            }
        }

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

#[cfg(feature = "multi-thread")]
pub async fn run_par<T: JsonRpcClient + Clone + 'static, U: Storage + Auth + Clone>(
    app: App<T, U>,
    criterias: Option<Criterias>,
) -> Result<()> {
    let handles = process(app, criterias).await?;
    futures::future::join_all(handles).await;

    Ok(())
}

pub async fn process_logs<T: JsonRpcClient + Clone + 'static, U: Storage + Auth + Clone>(
    app: App<T, U>,
    criterias: Option<Criterias>,
) -> Result<()> {
    let from = app.src_block();
    let to = app.dst_block();

    for target in from..=to {
        if let Some(crits) = criterias.as_ref() {
            let logs = app.fetch_logs(crits, target).await.unwrap();
            log::info!("{:?}", logs);

            for log in logs {
                println!("{:?}", log.clone());
                app.storage_conn()
                    .unwrap()
                    .insert_log(&log.into())
                    .await
                    .unwrap();
            }
        }
    }

    Ok(())
}

pub async fn process_blocks<T: JsonRpcClient + Clone + 'static, U: Storage + Auth + Clone>(
    app: App<T, U>,
) -> Result<()> {
    let from = app.src_block();
    let to = app.dst_block();

    for target in from..=to {
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

pub async fn stream_latest_blocks() {}

pub async fn stream_latest_logs() {}

pub async fn process<T: JsonRpcClient + Clone + 'static, U: Storage + Auth + Clone>(
    app: App<T, U>,
    criterias: Option<Criterias>,
) -> Result<Vec<JoinHandle<Result<()>>>> {
    let handles = vec![
        tokio::spawn(process_logs(app.clone(), criterias)),
        tokio::spawn(process_blocks(app)),
    ];

    Ok(handles)
}
