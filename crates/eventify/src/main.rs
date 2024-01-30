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
use configs::configs::{CollectorConfig, ManagerConfig, ServerConfig};
use core::{networks::eth::Eth, Collector, Manager, Store};
use primitives::{Criteria, NetworkKind};
//--

use std::{
    path::Path,
};
use tokio::{
    signal::ctrl_c,
    sync::{watch},
};

use clap::Parser;
use eyre::Result;

use sqlx::{migrate::Migrator, postgres::PgPoolOptions};
use tracing::{info, warn};
use tracing_subscriber::EnvFilter;

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
        .with_target(true)
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

            let node = match args.network {
                NetworkKind::Ethereum => Eth::new(args.node_url.clone()).await?,
            };
            let store = Store::new(args.database_url()).await;
            let redis = redis::Client::open(args.redis_url()).unwrap();

            let collector_config = CollectorConfig::new(args.network());
            let collector = Collector::new(collector_config, node, store, redis);
            let manager_config = if let Some(range) = args.block_range() {
                ManagerConfig::new(
                    args.skip_blocks(),
                    args.skip_transactions(),
                    args.skip_logs(),
                    criteria.clone(),
                    Some(range.into()),
                )
            } else {
                ManagerConfig::new(
                    args.skip_blocks(),
                    args.skip_transactions(),
                    args.skip_logs(),
                    criteria.clone(),
                    None,
                )
            };
            let manager = Manager::new(manager_config.clone(), collector.clone());

            let (sender, receiver) = watch::channel(false);
            tokio::spawn(async move {
                ctrl_c().await.unwrap();
                warn!("Received Ctrl-C signal, shutting down...");
                sender.send(true).unwrap();
            });
            match args.block_range() {
                Some(_) => {
                    if !args.skip_blocks() {
                        handles.push(manager.get_blocks_task(receiver.clone()).await?);
                    }

                    if !args.skip_transactions() {
                        handles.push(manager.get_transactions_task(receiver.clone()).await?);
                    }

                    if !args.skip_logs() {
                        handles.push(manager.get_logs_task(receiver.clone()).await?);
                    }
                }
                None => {
                    if !args.skip_blocks() {
                        handles.push(manager.stream_blocks_task(receiver.clone()).await?);
                    }

                    if !args.skip_transactions() {
                        handles.push(manager.stream_transactions_task(receiver.clone()).await?);
                    }

                    if !args.skip_logs() {
                        handles.push(manager.stream_logs_task(receiver.clone()).await?);
                    }
                }
            }

            if args.server_enabled() {
                handles.push(tokio::spawn(async move {
                    server::run(ServerConfig::from(args.clone())).await.unwrap()
                }));
            }

            futures::future::join_all(handles).await;
        }

        cmd::SubCommand::Db(_) => {
            unimplemented!("Database management.")
        }

        cmd::SubCommand::Config(_) => {
            unimplemented!("Config details.")
        }
    }

    Ok(())
}
