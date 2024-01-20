#![doc = include_str!("../README.md")]
#![warn(missing_debug_implementations, unreachable_pub, rustdoc::all)]
#![deny(unused_must_use, rust_2018_idioms)]

//--
pub mod cmd;
pub mod settings;
pub mod subcommands;

use eventify_core as idx;
use eventify_http_server as server;
use eventify_primitives as primitives;

use crate::cmd::Cmd;
use idx::{
    provider::{eth::Eth, NodeKind},
    storage::{Postgres, StorageClientKind, StorageKind},
    Collector, Manager, Run,
};
use primitives::{configs::ServerConfig, Criterias};
//--

use std::{path::Path, str::FromStr};

use clap::Parser;
use eyre::Result;
use sqlx::{migrate::Migrator, postgres::PgPoolOptions};
use tracing::{info, warn};
use tracing_subscriber::EnvFilter;
use url::Url;

async fn run_migrations(url: &str) -> Result<()> {
    let migrator = Migrator::new(Path::new("./migrations")).await?;
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
        cmd::SubCommand::Run(args) => {
            run_migrations(args.database_url()).await?;
            if args.only_migrations {
                return Ok(());
            }

            if !args.server_enabled() && !args.indexer_enabled() {
                warn!("Neither server nor indexer enabled, skipping");
            }

            let mut handles = vec![];

            if args.server_enabled() {
                handles.push(tokio::spawn(server::run(ServerConfig::from(args.clone()))));
            }

            if args.indexer_enabled() {
                // event criterias
                let criterias = args
                    .criterias_file()
                    .map(|file| Criterias::from_file(file.as_str()))
                    .transpose()?
                    .or_else(|| args.criterias_json());

                let node_client = match args.node {
                    NodeKind::Ethereum => Eth::new(args.node_url.clone()).await?,
                };

                let storage_client =
                    match StorageKind::from_str(Url::parse(args.database_url())?.scheme())? {
                        StorageKind::Postgres => {
                            StorageClientKind::Postgres(Postgres::new(args.database_url()).await)
                        }
                    };

                handles.push(tokio::spawn(Manager::run::<_, _, _>(
                    Collector::new(node_client, storage_client),
                    args.skip_transactions(),
                    args.skip_blocks(),
                    args.src_block(),
                    args.dst_block(),
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
