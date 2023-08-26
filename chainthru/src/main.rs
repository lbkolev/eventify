use clap::Parser;
use secrecy::{ExposeSecret, Secret};
use types::storage::Postgres;
use url::Url;
use web3::{
    transports::{Http, Ipc, WebSocket},
    types::{BlockId, BlockNumber},
};

use chainthru_index as indexer;
use chainthru_primitives as types;
use chainthru_server as server;
use indexer::app::App;

#[derive(Clone, Debug, Parser)]
#[command(name = "Chainthru")]
#[command(about = "Index Ethereum into a Postgresql database & serve it via an API server.")]
pub struct ChainthruSettings {
    #[arg(
        long,
        env = "CHAINTHRU_STORAGE_URL",
        help = "The database URL to connect to"
    )]
    pub storage_url: Secret<String>,

    #[arg(
        long,
        env = "CHAINTHRU_NODE_URL",
        help = "The Ethereum node URL to connect to",
        default_value = "http://localhost:8545"
    )]
    pub node_url: String,

    #[arg(
        long,
        env = "CHAINTHRU_SRC_BLOCK",
        help = "The block to begin the indexing from. Defaults to 0",
        default_value_t = 0
    )]
    pub src_block: u64,

    #[arg(
        long,
        env = "CHAINTHRU_DST_BLOCK",
        help = "The block to end the indexing at. Defaults to the latest block",
        default_value = None
    )]
    pub dst_block: Option<u64>,

    #[arg(
        long = "indexer.disabled",
        help = "Toggler disabling the indexer",
        default_value_t = false
    )]
    pub indexer_disabled: bool,

    #[arg(
        long = "indexer.threads",
        env = "CHAINTHRU_INDEXER_THREADS",
        help = "The number of threads to use for indexing",
        default_value_t = 1,
        value_parser = clap::value_parser!(u16).range(1..),
    )]
    pub indexer_threads: u16,

    #[arg(
        long = "server.disabled",
        help = "Toggler disabling the API server",
        default_value_t = false
    )]
    pub server_disabled: bool,

    #[arg(
        long = "server.host",
        env = "CHAINTHRU_SERVER_HOST",
        help = "The host to run the API server on",
        default_value = "0.0.0.0"
    )]
    pub server_host: String,

    #[arg(
        long = "server.port",
        env = "CHAINTHRU_SERVER_PORT",
        help = "The port to run the API server on",
        default_value_t = 6969,
        value_parser = clap::value_parser!(u16).range(1..),

    )]
    pub server_port: u16,

    #[arg(
        long = "server.threads",
        env = "CHAINTHRU_SERVER_THREADS",
        help = "The number of threads to use for the API server",
        default_value_t = num_cpus::get(),
    )]
    pub server_threads: usize,

    #[arg(
        long = "log.level",
        env = "RUST_LOG",
        help = "The log level to use",
        default_value = "warn",
        value_parser = clap::value_parser!(log::Level),
    )]
    pub log_level: log::Level,
}

impl From<ChainthruSettings> for server::Settings {
    fn from(settings: ChainthruSettings) -> Self {
        Self {
            database: types::DatabaseSettings::from(settings.storage_url.expose_secret().clone()),
            application: server::ApplicationSettings {
                host: settings.server_host,
                port: settings.server_port,
                worker_threads: settings.server_threads,
            },
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = ChainthruSettings::parse();
    let subscriber = chainthru_tracing::get_subscriber(
        "chainthru-server".into(),
        settings.log_level.as_str().into(),
        std::io::stdout,
    );
    chainthru_tracing::init_subscriber(subscriber);

    let server_settings = server::Settings::from(settings.clone());
    let mut handles = vec![];

    if !settings.server_disabled {
        handles.push(tokio::spawn(server::run(server_settings)));
    }

    if !settings.indexer_disabled {
        match Url::parse(&settings.node_url)?.scheme() {
            "http" | "https" => {
                tokio::spawn(indexer::run::<Http, Postgres>(
                    App::default()
                        .with_src_block(BlockId::Number(BlockNumber::Number(
                            settings.src_block.into(),
                        )))
                        .with_dst_block(BlockId::Number(BlockNumber::Number(
                            settings.dst_block.unwrap().into(),
                        )))
                        .with_storage(settings.storage_url.expose_secret())
                        .with_http(&settings.node_url),
                ));
            }
            "ws" | "wss" => {
                tokio::spawn(indexer::run::<WebSocket, Postgres>(
                    App::default()
                        .with_src_block(BlockId::Number(BlockNumber::Number(
                            settings.src_block.into(),
                        )))
                        .with_dst_block(BlockId::Number(BlockNumber::Number(
                            settings.dst_block.unwrap().into(),
                        )))
                        .with_storage(settings.storage_url.expose_secret())
                        .with_websocket(&settings.node_url)
                        .await,
                ));
            }
            "ipc" => {
                tokio::spawn(indexer::run::<Ipc, Postgres>(
                    App::default()
                        .with_src_block(BlockId::Number(BlockNumber::Number(
                            settings.src_block.into(),
                        )))
                        .with_dst_block(BlockId::Number(BlockNumber::Number(
                            settings.dst_block.unwrap().into(),
                        )))
                        .with_storage(settings.storage_url.expose_secret())
                        .with_ipc(&settings.node_url)
                        .await,
                ));
            }
            _ => {
                return Err("Invalid node URL scheme".into());
            }
        };
    }

    futures::future::join_all(handles).await;

    Ok(())
}
