#![doc = include_str!("../README.md")]
#![warn(missing_debug_implementations, unreachable_pub, rustdoc::all)]
#![deny(unused_must_use, rust_2018_idioms)]

use core::panic;
use std::path::Path;

use alloy_primitives::B256;
use clap::Parser;
use eyre::Result;

use sqlx::{migrate::Migrator, postgres::PgPoolOptions};
use tokio::{
    signal::{
        ctrl_c,
        unix::{signal, SignalKind},
    },
    sync::{
        mpsc::{self},
        watch,
    },
};

//--
pub mod cmd;
pub mod subcommands;

use crate::cmd::Cmd;
use eventify_configs::{
    configs::{ApplicationConfig, CollectorConfig, ManagerConfig},
    database::DatabaseConfig,
    Config,
};
use eventify_core::{
    networks::{ethereum::Eth, NetworkClient},
    Collector, Manager,
};
use eventify_engine::notify;
use eventify_primitives::{
    eth::{Block, Log, Transaction},
    networks::{NetworkKind, Resource},
    EmitT,
};
//--

use tracing::{debug, info, warn};
use tracing_subscriber::EnvFilter;

async fn run_migrations(url: &str) -> Result<()> {
    let migrator = Migrator::new(Path::new("./migrations")).await?;
    let pool = PgPoolOptions::new().max_connections(1).connect(url).await?;

    migrator.run(&pool).await?;

    Ok(())
}

async fn propagate(
    queue_url: &str,
    network: &NetworkKind,
    mut receiver: mpsc::Receiver<Resource<Block<B256>, Transaction, Log>>,
) -> Result<()> {
    let redis = redis::Client::open(queue_url)?;

    while let Some(rsrc) = receiver.recv().await {
        match rsrc {
            Resource::Block(ref block) => {
                rsrc.emit(&redis, network, &block).await?;
            }
            Resource::Tx(ref tx) => {
                rsrc.emit(&redis, network, &tx).await?;
            }
            Resource::Log(ref log) => {
                rsrc.emit(&redis, network, &log).await?;
            }
        }
    }

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
            let pool = PgPoolOptions::new()
                .acquire_timeout(std::time::Duration::from_secs(5))
                .connect_lazy_with(database_config.with_db());
            let (signal_sender, signal_receiver) = watch::channel(false);
            let mut tasks = vec![];

            if let Some(network) = config.network {
                if let Some(eth) = network.eth {
                    let network_kind = NetworkKind::Ethereum;
                    let (tx, rx) = mpsc::channel::<Resource<Block<B256>, Transaction, Log>>(1500);
                    let network_client = NetworkClient::new(eth.node_url).await?;

                    let collector_config = CollectorConfig::new(network_kind);
                    let collector = Collector::new(collector_config, Eth::new(network_client), tx);

                    let manager_config = ManagerConfig::new(config.collect);
                    let manager = Manager::new(manager_config.clone(), collector);
                    tasks.extend(manager.init_stream_tasks(signal_receiver).await?);

                    let propagate_queue_url = config.queue_url.clone();
                    let propagate_task = tokio::spawn(async move {
                        match propagate(&propagate_queue_url, &network_kind, rx).await {
                            Ok(_) => println!("Propagation completed successfully."),
                            Err(e) => println!("Propagation failed with error: {:?}", e),
                        }
                    });
                    tasks.push(propagate_task);
                }
            }

            if config.notify {
                let (notify_queue_url, notify_pool) = (config.queue_url.clone(), pool.clone());
                let notify_task = tokio::spawn(async move {
                    match notify(notify_queue_url, notify_pool).await {
                        Ok(_) => println!("Notify completed successfully."),
                        Err(e) => println!("Notify failed with error: {:?}", e),
                    }
                });
                tasks.push(notify_task);
            }

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
                    std::process::exit(0);
                }
                _ = tokio::spawn(async move {
                    ctrl_c().await.unwrap();
                    signal_sender.send(true).unwrap();
                }) => {
                    warn!("Received SIGINT, shutting down..");
                    tokio::time::sleep(tokio::time::Duration::from_secs(6)).await; // give the streaming threads time to gracefully wind down
                    std::process::exit(0);
                }
                _ = tokio::spawn(async move {
                    let mut stream = signal(SignalKind::terminate()).unwrap();

                    loop {
                        stream.recv().await;
                    }
                }) => {
                    warn!("Received SIGTERM, shutting down..");
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
