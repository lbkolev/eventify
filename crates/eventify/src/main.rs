#![doc = include_str!("../README.md")]
#![warn(missing_debug_implementations, unreachable_pub, rustdoc::all)]
#![deny(unused_must_use, rust_2018_idioms)]

use std::path::Path;

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
use tracing::{debug, error, info, warn};
use tracing_subscriber::EnvFilter;

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
    networks::{
        arbitrum::Arbitrum, avalanche::Avalanche, base::Base, bsc::Bsc, ethereum::Eth,
        linea::Linea, optimism::Optimism, polygon::Polygon, zksync::Zksync,
    },
    Manager,
};
use eventify_primitives::{
    networks::{
        arbitrum::{ArbitrumBlock, ArbitrumLog},
        avalanche::{AvalancheBlock, AvalancheLog},
        base::{BaseBlock, BaseLog},
        bsc::{BscBlock, BscLog},
        ethereum::{EthBlock, EthLog},
        linea::{LineaBlock, LineaLog},
        optimism::{OptimismBlock, OptimismLog},
        polygon::{PolygonBlock, PolygonLog},
        zksync::{ZksyncBlock, ZksyncLog},
        NetworkKind, Resource,
    },
    BlockT, EmitT, LogT,
};
//--

async fn run_migrations(url: &str) -> Result<()> {
    let migrator = Migrator::new(Path::new("./migrations")).await?;
    let pool = PgPoolOptions::new().max_connections(1).connect(url).await?;

    migrator.run(&pool).await?;

    Ok(())
}

async fn propagate<B: BlockT, L: LogT>(
    queue_url: &str,
    network: &NetworkKind,
    mut receiver: mpsc::Receiver<Resource<B, L>>,
) -> Result<()> {
    let redis = redis::Client::open(queue_url)?;

    while let Some(rsrc) = receiver.recv().await {
        rsrc.emit(&redis, network).await?;
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

            if let Some(eth) = config.network.eth {
                let network_kind = NetworkKind::Ethereum;
                let (tx, rx) = mpsc::channel::<Resource<EthBlock, EthLog>>(1500);

                let collector_config = CollectorConfig::new(network_kind, eth.node_url.clone());
                let manager_config = ManagerConfig::new(config.collect.clone());
                let manager: Manager<Eth> = Manager::new(manager_config, collector_config, tx);
                tasks.extend(manager.init_stream_tasks(signal_receiver.clone()).await?);

                let propagate_queue_url = config.queue_url.clone();
                let propagate_task = tokio::spawn(async move {
                    match propagate(&propagate_queue_url, &network_kind, rx).await {
                        Ok(_) => info!("Propagation completed successfully."),
                        Err(e) => error!("Propagation failed with error: {:?}", e),
                    }
                });
                tasks.push(propagate_task);
            }

            if let Some(zksync) = config.network.zksync {
                let network_kind = NetworkKind::Zksync;
                let (tx, rx) = mpsc::channel::<Resource<ZksyncBlock, ZksyncLog>>(1500);

                let collector_config = CollectorConfig::new(network_kind, zksync.node_url.clone());
                let manager_config = ManagerConfig::new(config.collect.clone());
                let manager: Manager<Zksync> = Manager::new(manager_config, collector_config, tx);
                tasks.extend(manager.init_stream_tasks(signal_receiver.clone()).await?);

                let propagate_queue_url = config.queue_url.clone();
                let propagate_task = tokio::spawn(async move {
                    match propagate(&propagate_queue_url, &network_kind, rx).await {
                        Ok(_) => info!("Propagation completed successfully."),
                        Err(e) => error!("Propagation failed with error: {:?}", e),
                    }
                });
                tasks.push(propagate_task);
            }

            if let Some(polygon) = config.network.polygon {
                let network_kind = NetworkKind::Polygon;
                let (tx, rx) = mpsc::channel::<Resource<PolygonBlock, PolygonLog>>(1500);

                let collector_config = CollectorConfig::new(network_kind, polygon.node_url.clone());
                let manager_config = ManagerConfig::new(config.collect.clone());
                let manager: Manager<Polygon> = Manager::new(manager_config, collector_config, tx);
                tasks.extend(manager.init_stream_tasks(signal_receiver.clone()).await?);

                let propagate_queue_url = config.queue_url.clone();
                let propagate_task = tokio::spawn(async move {
                    match propagate(&propagate_queue_url, &network_kind, rx).await {
                        Ok(_) => info!("Propagation completed successfully."),
                        Err(e) => error!("Propagation failed with error: {:?}", e),
                    }
                });
                tasks.push(propagate_task);
            }

            if let Some(optimism) = config.network.optimism {
                let network_kind = NetworkKind::Optimism;
                let (tx, rx) = mpsc::channel::<Resource<OptimismBlock, OptimismLog>>(1500);

                let collector_config =
                    CollectorConfig::new(network_kind, optimism.node_url.clone());
                let manager_config = ManagerConfig::new(config.collect.clone());
                let manager: Manager<Optimism> = Manager::new(manager_config, collector_config, tx);
                tasks.extend(manager.init_stream_tasks(signal_receiver.clone()).await?);

                let propagate_queue_url = config.queue_url.clone();
                let propagate_task = tokio::spawn(async move {
                    match propagate(&propagate_queue_url, &network_kind, rx).await {
                        Ok(_) => info!("Propagation completed successfully."),
                        Err(e) => error!("Propagation failed with error: {:?}", e),
                    }
                });
                tasks.push(propagate_task);
            }

            if let Some(arbitrum) = config.network.arbitrum {
                let network_kind = NetworkKind::Arbitrum;
                let (tx, rx) = mpsc::channel::<Resource<ArbitrumBlock, ArbitrumLog>>(1500);

                let collector_config =
                    CollectorConfig::new(network_kind, arbitrum.node_url.clone());
                let manager_config = ManagerConfig::new(config.collect.clone());
                let manager: Manager<Arbitrum> = Manager::new(manager_config, collector_config, tx);
                tasks.extend(manager.init_stream_tasks(signal_receiver.clone()).await?);

                let propagate_queue_url = config.queue_url.clone();
                let propagate_task = tokio::spawn(async move {
                    match propagate(&propagate_queue_url, &network_kind, rx).await {
                        Ok(_) => info!("Propagation completed successfully."),
                        Err(e) => error!("Propagation failed with error: {:?}", e),
                    }
                });
                tasks.push(propagate_task);
            }

            if let Some(linea) = config.network.linea {
                let network_kind = NetworkKind::Linea;
                let (tx, rx) = mpsc::channel::<Resource<LineaBlock, LineaLog>>(1500);

                let collector_config = CollectorConfig::new(network_kind, linea.node_url.clone());
                let manager_config = ManagerConfig::new(config.collect.clone());
                let manager: Manager<Linea> = Manager::new(manager_config, collector_config, tx);
                tasks.extend(manager.init_stream_tasks(signal_receiver.clone()).await?);

                let propagate_queue_url = config.queue_url.clone();
                let propagate_task = tokio::spawn(async move {
                    match propagate(&propagate_queue_url, &network_kind, rx).await {
                        Ok(_) => info!("Propagation completed successfully."),
                        Err(e) => error!("Propagation failed with error: {:?}", e),
                    }
                });
                tasks.push(propagate_task);
            }

            if let Some(avalanche) = config.network.avalanche {
                let network_kind = NetworkKind::Avalanche;
                let (tx, rx) = mpsc::channel::<Resource<AvalancheBlock, AvalancheLog>>(1500);

                let collector_config =
                    CollectorConfig::new(network_kind, avalanche.node_url.clone());
                let manager_config = ManagerConfig::new(config.collect.clone());
                let manager: Manager<Avalanche> =
                    Manager::new(manager_config, collector_config, tx);
                tasks.extend(manager.init_stream_tasks(signal_receiver.clone()).await?);

                let propagate_queue_url = config.queue_url.clone();
                let propagate_task = tokio::spawn(async move {
                    match propagate(&propagate_queue_url, &network_kind, rx).await {
                        Ok(_) => info!("Propagation completed successfully."),
                        Err(e) => error!("Propagation failed with error: {:?}", e),
                    }
                });
                tasks.push(propagate_task);
            }

            if let Some(bsc) = config.network.bsc {
                let network_kind = NetworkKind::Bsc;
                let (tx, rx) = mpsc::channel::<Resource<BscBlock, BscLog>>(1500);

                let collector_config = CollectorConfig::new(network_kind, bsc.node_url.clone());
                let manager_config = ManagerConfig::new(config.collect.clone());
                let manager: Manager<Bsc> = Manager::new(manager_config, collector_config, tx);
                tasks.extend(manager.init_stream_tasks(signal_receiver.clone()).await?);

                let propagate_queue_url = config.queue_url.clone();
                let propagate_task = tokio::spawn(async move {
                    match propagate(&propagate_queue_url, &network_kind, rx).await {
                        Ok(_) => info!("Propagation completed successfully."),
                        Err(e) => error!("Propagation failed with error: {:?}", e),
                    }
                });
                tasks.push(propagate_task);
            }

            if let Some(base) = config.network.base {
                let network_kind = NetworkKind::Base;
                let (tx, rx) = mpsc::channel::<Resource<BaseBlock, BaseLog>>(1500);

                let collector_config = CollectorConfig::new(network_kind, base.node_url.clone());
                let manager_config = ManagerConfig::new(config.collect.clone());
                let manager: Manager<Base> = Manager::new(manager_config, collector_config, tx);
                tasks.extend(manager.init_stream_tasks(signal_receiver.clone()).await?);

                let propagate_queue_url = config.queue_url.clone();
                let propagate_task = tokio::spawn(async move {
                    match propagate(&propagate_queue_url, &network_kind, rx).await {
                        Ok(_) => info!("Propagation completed successfully."),
                        Err(e) => error!("Propagation failed with error: {:?}", e),
                    }
                });
                tasks.push(propagate_task);
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

            Ok(())
        }
    }
}
