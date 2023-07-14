use clap::Parser;
use web3::types::{BlockId, BlockNumber};

#[derive(Parser, Debug)]
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
    node_url: String,

    #[arg(
        long,
        env = "CHAINTHRU_DATABASE_URL",
        help = "The database URL to connect to."
    )]
    database_url: String,

    #[arg(
        long,
        env = "CHAINTHRU_FROM_BLOCK",
        help = "The block to begin the indexing from. Defaults to 0.",
        default_value_t = 0
    )]
    from_block: u64,

    #[arg(
        long,
        env = "CHAINTHRU_TO_BLOCK",
        help = "The block to end the indexing at. Defaults to the latest block."
    )]
    to_block: Option<u64>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = Settings::parse();

    let db_conn = sqlx::PgPool::connect(&settings.database_url).await?;
    sqlx::migrate!("./migrations").run(&db_conn).await?;

    // Connect to the Ethereum node
    let web3 = web3::Web3::new(web3::transports::Http::new(&settings.node_url)?);

    // Specify the block number or block hash to retrieve
    let block_number = BlockNumber::Number(1345340.into()); // Replace with your desired block number

    // Retrieve the block with transactions
    let block_with_txs = web3
        .eth()
        .block_with_txs(BlockId::Number(block_number))
        .await?;

    println!("Block: {:?}", block_with_txs);

    Ok(())
}
