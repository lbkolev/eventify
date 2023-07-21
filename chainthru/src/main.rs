use clap::Parser;

use chainthru_index::IndexSettings;

#[derive(Parser)]
#[command(name = "Chainthru")]
#[command(author = "Lachezar Kolev <lachezarkolevgg@gmail.com>")]
#[command(version = "0.1")]
#[command(about = "Index Ethereum into a Postgresql database & optionally serve it via an API.")]
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
}

impl From<Settings> for IndexSettings {
    fn from(settings: Settings) -> Self {
        Self {
            database_url: settings.database_url,
            node_url: settings.node_url,
            from_block: settings.from_block,
            to_block: settings.to_block,
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = Settings::parse();
    let index_settings = IndexSettings::from(settings);

    if index_settings.from_block > index_settings.to_block.unwrap_or(0) {
        panic!("The from block cannot be greater than the to block.");
    }

    let _ = tokio::task::spawn(chainthru_index::run(index_settings.clone())).await;

    Ok(())
}
