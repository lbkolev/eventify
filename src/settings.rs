use clap::Parser;

#[derive(Parser)]
#[command(name = "Chainthru")]
#[command(author = "Lachezar Kolev <lachezarkolevgg@gmail.com>")]
#[command(version = "0.1")]
#[command(about = "Index Ethereum into a database.")]
pub struct Settings {
    #[arg(
        long,
        env = "CHAINTHRU_NODE_URL",
        help = "The Ethereum node URL to connect to.",
        default_value = "http://localhost:8545"
    )]
    pub node_url: String,

    #[arg(
        long,
        env = "CHAINTHRU_DATABASE_URL",
        help = "The database URL to connect to."
    )]
    pub database_url: String,

    #[arg(
        long,
        env = "CHAINTHRU_FROM_BLOCK",
        help = "The block to begin the indexing from. Defaults to 0.",
        default_value_t = 0
    )]
    pub from_block: u64,

    #[arg(
        long,
        env = "CHAINTHRU_TO_BLOCK",
        help = "The block to end the indexing at. Defaults to the latest block.",
        default_value = None
    )]
    pub to_block: Option<u64>,
}
