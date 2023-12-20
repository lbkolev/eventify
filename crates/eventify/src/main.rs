#![doc = include_str!("../README.md")]
#![warn(missing_debug_implementations, unreachable_pub, rustdoc::all)]
#![deny(unused_must_use, rust_2018_idioms)]

//----
pub mod error;
pub mod settings;
pub mod subcommands;

use error::Error;
use eventify_http_server as server;
use eventify_idx as indexer;
use eventify_primitives as types;

use crate::settings::Settings;
use indexer::{
    providers::{EthHttp, EthIpc, EthWs},
    types::provider::NodeProvider,
    Collector, Manager, Run,
};
use types::{config::ServerConfig, storage::Postgres, Criterias};

pub type Result<T> = std::result::Result<T, Error>;
//----

use std::path::Path;

use clap::Parser;
use futures::TryFutureExt;
use sqlx::{migrate::Migrator, postgres::PgPoolOptions};
use tracing::{subscriber::set_global_default, Subscriber};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{fmt::MakeWriter, layer::SubscriberExt, EnvFilter, Registry};
use url::Url;

fn get_subscriber<Sink>(
    name: String,
    env_filter: String,
    sink: Sink,
) -> impl Subscriber + Sync + Send
where
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    let env_filter = EnvFilter::new(env_filter);
    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(BunyanFormattingLayer::new(name, sink))
}

fn init_subscriber(subscriber: impl Subscriber + Sync + Send) {
    LogTracer::init().expect("Failed to set logger");
    set_global_default(subscriber).expect("Failed to set subscriber");
}

async fn run_migrations(url: &str) -> Result<()> {
    let migrator = Migrator::new(Path::new("./migrations/rdms/postgres")).await?;
    let pool = PgPoolOptions::new().max_connections(1).connect(url).await?;

    migrator.run(&pool).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let settings = Settings::parse();
    let subscriber = get_subscriber(
        "eventify".into(),
        settings.log_level.as_str().into(),
        std::io::stdout,
    );
    init_subscriber(subscriber);
    log::info!("{:#?}", settings);

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
