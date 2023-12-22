#![doc = include_str!("../README.md")]
#![warn(missing_debug_implementations, unreachable_pub, rustdoc::all)]
#![deny(unused_must_use, rust_2018_idioms)]

//----
pub mod error;
pub mod settings;
pub mod subcommands;

use error::Error;
use eventify_http_server as server;
use eventify_idx as idx;
use eventify_primitives as primitives;

use crate::settings::Settings;
use idx::{
    providers::{storage::Postgres, EthHttp, EthIpc, EthWs},
    types::NodeProvider,
    Collector, Manager, Run,
};
use primitives::{configs::ServerConfig, Criterias};
use tracing::info;

pub type Result<T> = std::result::Result<T, Error>;
//----

use std::path::Path;

use clap::Parser;
use futures::TryFutureExt;
use sqlx::{migrate::Migrator, postgres::PgPoolOptions};
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
    let settings = Settings::parse();
    tracing_subscriber::fmt()
        .with_thread_ids(true)
        .with_env_filter(EnvFilter::builder().from_env_lossy())
        .init();

    info!(target:"eventify::cli", ?settings);
    match settings.cmd {
        settings::SubCommand::Run(settings) => {
            run_migrations(settings.database_url()).await?;
            if settings.only_migrations {
                return Ok(());
            }

            let mut handles = vec![];

            if settings.server_enabled() {
                handles.push(tokio::spawn(
                    server::run(ServerConfig::from(settings.clone())).map_err(Error::from),
                ));
            }

            if settings.indexer_enabled() {
                // event criterias
                let criterias = settings
                    .criterias_file()
                    .map(|file| Criterias::from_file(file.as_str()))
                    .transpose()?
                    .or_else(|| settings.criterias_json());

                match Url::parse(&settings.node_url)?.scheme() {
                    "http" | "https" => {
                        handles.push(tokio::spawn(
                            Manager::run::<_, _, Error>(
                                Collector::new(
                                    EthHttp::new(&settings.node_url).await?,
                                    Postgres::new(settings.database_url()),
                                ),
                                settings.src_block(),
                                settings.dst_block(),
                                criterias,
                            )
                            .map_err(Error::from),
                        ));
                    }
                    "ws" | "wss" => {
                        handles.push(tokio::spawn(
                            Manager::run::<_, _, Error>(
                                Collector::new(
                                    EthWs::new(&settings.node_url).await?,
                                    Postgres::new(settings.database_url()),
                                ),
                                settings.src_block(),
                                settings.dst_block(),
                                criterias,
                            )
                            .map_err(Error::from),
                        ));
                    }
                    "ipc" => {
                        handles.push(tokio::spawn(
                            Manager::run::<_, _, Error>(
                                Collector::new(
                                    EthIpc::new(&settings.node_url).await?,
                                    Postgres::new(settings.database_url()),
                                ),
                                settings.src_block(),
                                settings.dst_block(),
                                criterias,
                            )
                            .map_err(Error::from),
                        ));
                    }
                    _ => {
                        return Err(Error::NodeURLScheme(settings.node_url));
                    }
                };
            }

            futures::future::join_all(handles).await;
        }

        settings::SubCommand::Db(_) => {
            unimplemented!("Database management.")
        }

        settings::SubCommand::Config(_) => {
            unimplemented!("Configuration management.")
        }
    }

    Ok(())
}
