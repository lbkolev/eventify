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
use eventify_core::{
    networks::{eth::Eth, NetworkClient},
    Collector, Manager, Storage,
};
use eventify_primitives::networks::{eth::Criteria, NetworkKind};
//--

use std::path::Path;

use clap::Parser;
use eyre::Result;
use sqlx::{migrate::Migrator, postgres::PgPoolOptions};
use tokio::{
    signal::{
        ctrl_c,
        unix::{signal, SignalKind},
    },
    sync::watch,
};
use tracing::{debug, info, warn};

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
                Config::from(*args)
            };
            debug!(target:"eventify::cli", ?config);

            let database_config = DatabaseConfig::from(config.database_url);
            let store = Storage::connect(database_config.clone()).await;
            let pool = store.inner().clone();

            let (signal_sender, signal_receiver) = watch::channel(false);
            let redis = redis::Client::open(config.queue_url)?;

            let collector_config = CollectorConfig::new(NetworkKind::Ethereum);
            let network_client = NetworkClient::new(config.network.eth.unwrap().node_url).await?;
            let collector =
                Collector::new(collector_config, Eth::new(network_client), store, redis);

            let manager_config = ManagerConfig::new(config.collect);
            let manager = Manager::new(manager_config.clone(), collector);

            let mut tasks = match config.mode.kind {
                ModeKind::Batch => {
                    if let (Some(src), Some(dst)) = (config.mode.src, config.mode.dst) {
                        manager
                            .init_collect_tasks(
                                signal_receiver,
                                Criteria::new(src, dst, None, None),
                            )
                            .await?
                    } else {
                        vec![]
                    }
                }
                ModeKind::Stream => manager.init_stream_tasks(signal_receiver).await?,
            };

            if let Some(server_config) = config.server {
                let app_config = ApplicationConfig {
                    database: database_config,
                    server: server_config,
                };
                tasks.push(tokio::spawn(async move {
                    eventify_http_server::run(app_config, pool).await.unwrap()
                }))
            }

            tokio::select! {
                _ = futures::future::select_all(tasks) => {
                    info!("Tasks finished.");
                }
                _ = tokio::spawn(async move {
                    ctrl_c().await.unwrap();
                    signal_sender.send(true).unwrap();
                }) => {
                    warn!("Received SIGINT, shutting down...");
                    tokio::time::sleep(tokio::time::Duration::from_secs(6)).await; // give the streaming threads time to gracefully wind down
                }
                _ = tokio::spawn(async move {
                    let mut stream = signal(SignalKind::terminate()).unwrap();

                    loop {
                        stream.recv().await;
                    }
                }) => {
                    warn!("Received SIGTERM, shutting down...");
                }
            }
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
