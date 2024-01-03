#![doc = include_str!("../README.md")]
#![warn(missing_debug_implementations, unreachable_pub, rustdoc::all)]
#![deny(unused_must_use, rust_2018_idioms)]

pub use ethers_providers::{Middleware, StreamExt};
//-- crate-specific
pub mod cmd;
pub mod settings;
pub mod subcommands;

use eventify_http_server as server;
use eventify_idx as idx;
use eventify_primitives as primitives;

use crate::cmd::Cmd;
use idx::{
    clients::{storage::Postgres, EthHttp, EthIpc, EthWs, NodeClientKind},
    ChainKind, Collector, Manager, Run,
};
use primitives::{configs::ServerConfig, Criterias};
//--

use std::path::Path;

use clap::Parser;
use eyre::Result;
use sqlx::{migrate::Migrator, postgres::PgPoolOptions};
use tracing::info;
use tracing_subscriber::EnvFilter;
use url::Url;

async fn run_migrations(url: &str) -> Result<()> {
    let migrator = Migrator::new(Path::new("./migrations/rdms/postgres")).await?;
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
            run_migrations(cmd.database_url()).await?;
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

                let node_client = match cmd.chain {
                    ChainKind::Ethereum => match Url::parse(&cmd.node_url)?.scheme() {
                        "ipc" => NodeClientKind::EthIpc(EthIpc::new(&cmd.node_url).await),
                        "ws" | "wss" => NodeClientKind::EthWs(EthWs::new(&cmd.node_url).await),
                        _ => NodeClientKind::EthHttp(EthHttp::new(&cmd.node_url).await),
                    },
                };

                handles.push(tokio::spawn(Manager::run::<_, _, _>(
                    Collector::new(node_client, Postgres::new(cmd.database_url()).await),
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
