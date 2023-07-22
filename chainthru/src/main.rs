use clap::Parser;
use env_logger::Builder;

use chainthru_index as index;
use chainthru_server as server;

#[derive(Clone, Debug, Parser)]
#[command(name = "Chainthru")]
#[command(author = "Lachezar Kolev <lachezarkolevgg@gmail.com>")]
#[command(version = "0.1")]
#[command(about = "Index Ethereum into a Postgresql database & serve it via an API server.")]
pub struct Settings {
    #[arg(
        long,
        env = "CHAINTHRU_NODE_URL",
        help = "The Ethereum node URL to connect to",
        default_value = "http://localhost:8545"
    )]
    pub node_url: String,

    #[arg(
        long,
        env = "CHAINTHRU_DATABASE_URL",
        help = "The database URL to connect to"
    )]
    pub database_url: String,

    #[arg(
        long,
        env = "CHAINTHRU_FROM_BLOCK",
        help = "The block to begin the indexing from. Defaults to 0",
        default_value_t = 0
    )]
    pub from_block: u64,

    #[arg(
        long,
        env = "CHAINTHRU_TO_BLOCK",
        help = "The block to end the indexing at. Defaults to the latest block",
        default_value = None
    )]
    pub to_block: Option<u64>,

    #[arg(
        long = "indexer.threads",
        env = "CHAINTHRU_INDEXER_THREADS",
        help = "The number of threads to use for indexing",
        default_value_t = 1,
        value_parser = clap::value_parser!(u16).range(1..),
    )]
    pub indexer_threads: u16,

    #[arg(
        long = "server.enabled",
        help = "Toggler enabling the API server",
        default_value_t = false
    )]
    pub server: bool,

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

impl From<Settings> for index::Settings {
    fn from(settings: Settings) -> Self {
        Self {
            database_url: settings.database_url,
            node_url: settings.node_url,
            from_block: settings.from_block,
            to_block: settings.to_block,
        }
    }
}

impl From<Settings> for server::AppSettings {
    fn from(settings: Settings) -> Self {
        Self {
            host: settings.server_host,
            port: settings.server_port,
            database_url: settings.database_url,
            worker_threads: settings.server_threads,
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = Settings::parse();
    Builder::new()
        .parse_filters(settings.log_level.as_str())
        .init();
    log::info!("Settings: {:?}", settings);

    let index_settings = index::Settings::from(settings.clone());
    let server_settings = server::AppSettings::from(settings.clone());

    let mut handles = vec![];
    if settings.server {
        handles.push(tokio::spawn(server::run(server_settings).await?));
    }

    if index_settings.from_block > index_settings.to_block.unwrap_or(0) {
        panic!("The from block cannot be greater than the to block.");
    }

    tokio::spawn(index::run(index_settings));
    futures::future::join_all(handles).await;

    Ok(())
}
