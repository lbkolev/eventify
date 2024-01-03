#![doc = include_str!("../README.md")]
#![warn(missing_debug_implementations, unreachable_pub, rustdoc::all)]
#![deny(unused_must_use, rust_2018_idioms)]

//-- crate-specific
pub mod cmd;
pub mod settings;
pub mod subcommands;

use eventify_http_server as server;
use eventify_idx as idx;
use eventify_primitives as primitives;

use crate::cmd::Cmd;
use idx::{
    clients::{
        node::{NodeClientKind, NodeKind},
        storage::{Postgres, StorageClientKind, StorageKind},
        EthHttp, EthIpc, EthWs,
    },
    Collector, Manager, Run,
};
use primitives::{configs::ServerConfig, Criterias};
//--

use std::{path::Path, str::FromStr};

use clap::Parser;
use eyre::Result;
use sqlx::{migrate::Migrator, postgres::PgPoolOptions};
use tracing::info;
use tracing_subscriber::EnvFilter;
use url::Url;

async fn run_migrations(url: &str, kind: StorageKind) -> Result<()> {
    let migrator = Migrator::new(Path::new(&format!("./migrations/rdms/{}", kind))).await?;
    let pool = PgPoolOptions::new().max_connections(1).connect(url).await?;

    migrator.run(&pool).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let cmd = Cmd::parse();
    tracing_subscriber::fmt()
        .with_thread_ids(true)
        .with_env_filter(EnvFilter::builder().from_env_lossy())
        .init();

    info!(target:"eventify::cli", ?cmd);
    match cmd.subcmd {
        cmd::SubCommand::Run(cmd) => {
            run_migrations(
                cmd.database_url(),
                StorageKind::from_str(Url::parse(cmd.database_url())?.scheme())?,
            )
            .await?;
            if cmd.only_migrations {
                return Ok(());
            }

            let mut handles = vec![];

            if cmd.server_enabled() {
                handles.push(tokio::spawn(server::run(ServerConfig::from(cmd.clone()))));
            }

            if cmd.indexer_enabled() {
                // event criterias
                let criterias = cmd
                    .criterias_file()
                    .map(|file| Criterias::from_file(file.as_str()))
                    .transpose()?
                    .or_else(|| cmd.criterias_json());

                let node_client = match cmd.node {
                    NodeKind::Ethereum => match Url::parse(&cmd.node_url)?.scheme() {
                        "ipc" => NodeClientKind::EthIpc(EthIpc::new(&cmd.node_url).await),
                        "ws" | "wss" => NodeClientKind::EthWs(EthWs::new(&cmd.node_url).await),
                        _ => NodeClientKind::EthHttp(EthHttp::new(&cmd.node_url).await),
                    },
                };

                let storage_client =
                    match StorageKind::from_str(Url::parse(cmd.database_url())?.scheme())? {
                        StorageKind::Postgres => {
                            StorageClientKind::Postgres(Postgres::new(cmd.database_url()).await)
                        }
                    };

                handles.push(tokio::spawn(Manager::run::<_, _, _>(
                    Collector::new(node_client, storage_client),
                    cmd.skip_transactions(),
                    cmd.skip_blocks(),
                    cmd.src_block(),
                    cmd.dst_block(),
                    criterias,
                )));
            }

            futures::future::join_all(handles).await;
        }

        cmd::SubCommand::Stream(_) => {
            unimplemented!("Stream.")
        }

        cmd::SubCommand::Db(_) => {
            unimplemented!("Database management.")
        }

        cmd::SubCommand::Config(_) => {
            unimplemented!("Configuration management.")
        }
    }

    Ok(())
}
