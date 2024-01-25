#![doc = include_str!("../README.md")]
#![warn(missing_debug_implementations, unreachable_pub, rustdoc::all)]
#![deny(unused_must_use, rust_2018_idioms)]

//--
pub mod cmd;
pub mod settings;
pub mod subcommands;

use eventify_configs as configs;
use eventify_core as core;
use eventify_http_server as server;
use eventify_primitives as primitives;

use crate::cmd::Cmd;
use configs::configs::{ManagerConfig, ServerConfig};
use core::{
    provider::{eth::Eth, NodeKind},
    storage::{Postgres, StorageClientKind, StorageKind},
    Collector, Manager,
};
use primitives::Criteria;

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
            if cmd.only_migrations {
                return Ok(());
            }

            let mut handles = vec![];

            // event criteria
            let criteria = args
                .criteria_file()
                .map(|file| Criteria::from_file(file.as_str()))
                .transpose()?
                .or_else(|| args.criteria_json());

            let node_client = match args.node {
                NodeKind::Ethereum => Eth::new(args.node_url.clone()).await?,
            };

            let storage_client =
                match StorageKind::from_str(Url::parse(args.database_url())?.scheme())? {
                    StorageKind::Postgres => {
                        StorageClientKind::Postgres(Postgres::new(args.database_url()).await)
                    }
                };

            match args.block_range() {
                Some(range) => {
                    let config = ManagerConfig::new(
                        args.skip_blocks(),
                        args.skip_transactions(),
                        args.skip_logs(),
                        criteria.clone(),
                        Some(range.into()),
                    );
                    let collector = Collector::new(node_client, storage_client);

                    if !args.skip_blocks() {
                        handles.push(
                            Manager::new(config.clone(), collector.clone())
                                .get_blocks_task()
                                .await?,
                        );
                    }

                    if !args.skip_transactions() {
                        handles.push(
                            Manager::new(config.clone(), collector.clone())
                                .get_transactions_task()
                                .await?,
                        );
                    }

                    if !args.skip_logs() {
                        handles.push(
                            Manager::new(config.clone(), collector.clone())
                                .get_logs_task()
                                .await?,
                        );
                    }
                }

                None => {
                    let config = ManagerConfig::new(
                        args.skip_blocks(),
                        args.skip_transactions(),
                        args.skip_logs(),
                        criteria.clone(),
                        None,
                    );
                    let collector = Collector::new(node_client, storage_client);

                    if !args.skip_blocks() {
                        handles.push(
                            Manager::new(config.clone(), collector.clone())
                                .stream_blocks_task()
                                .await?,
                        );
                    }

                    if !args.skip_transactions() {
                        handles.push(
                            Manager::new(config.clone(), collector.clone())
                                .stream_transactions_task()
                                .await?,
                        );
                    }

                    if !args.skip_logs() {
                        handles.push(
                            Manager::new(config.clone(), collector.clone())
                                .stream_logs_task()
                                .await?,
                        );
                    }
                }
            }

            //if args.server_enabled() {
            //    handles.push(tokio::spawn(server::run(ServerConfig::from(args.clone()))));
            //}

            futures::future::join_all(handles).await;
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
