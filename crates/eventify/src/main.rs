#![doc = include_str!("../README.md")]
#![warn(missing_debug_implementations, unreachable_pub, rustdoc::all)]
#![deny(unused_must_use, rust_2018_idioms)]

//--
pub mod cmd;
pub mod subcommands;

use crate::cmd::Cmd;
use eventify_configs::{
    configs::{ApplicationConfig, CollectorConfig, ManagerConfig},
    database::DatabaseConfig,
    Config, ModeKind,
};
use eventify_core::{networks::eth::Eth, Collector, Manager, Store};
use eventify_primitives::{Criteria, NetworkKind};
//--

use std::path::Path;

use clap::Parser;
use eyre::Result;
use sqlx::{migrate::Migrator, postgres::PgPoolOptions};
use tokio::{signal::ctrl_c, sync::watch};
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

            let config: Config = if let Some(file) = args.config {
                info!(target:"eventify::cli", "Loading config from {}", file);
                toml::from_str(std::fs::read_to_string(file.as_str())?.as_str())?
            } else {
                Config::from(args)
            };

            let (signal_sender, signal_receiver) = watch::channel(false);
            tokio::spawn(async move {
                ctrl_c().await.unwrap();
                warn!("Received Ctrl-C signal, shutting down...");
                signal_sender.send(true).unwrap();
            });

            let store = Store::new(config.database_url.as_str()).await;
            let redis = redis::Client::open(config.redis_url).unwrap();

            let collector_config = CollectorConfig::new(NetworkKind::Ethereum);
            let collector = Collector::new(
                collector_config,
                Eth::new(config.network.eth.unwrap().node_url).await?,
                store,
                redis,
            );

            let manager_config = ManagerConfig::new(config.collect);
            let manager = Manager::new(manager_config.clone(), collector);

            let mut tasks = match config.mode.kind {
                ModeKind::Batch => {
                    manager
                        .init_collect_tasks(
                            signal_receiver,
                            config.mode.src.unwrap(),
                            config.mode.dst.unwrap(),
                            Criteria::default(),
                        )
                        .await?
                }
                ModeKind::Stream => manager.init_stream_tasks(signal_receiver).await?,
            };

            if let Some(server_config) = config.server {
                let app_config = ApplicationConfig {
                    database: DatabaseConfig::from(config.database_url.as_str()),
                    server: server_config,
                };
                tasks.push(tokio::spawn(async move {
                    eventify_http_server::run(app_config).await.unwrap()
                }))
            }

            futures::future::join_all(tasks).await;
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
