use clap::Parser;
use ethereum_types::{H160, U256};
use web3::types::BlockId;

use chainthru_index::transaction::erc20::{self, TRANSFER_SIGNATURE};

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

    #[arg(
        long,
        help = "Boolean enabling/disabling the API server.",
        default_value_t = false
    )]
    pub server: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = Settings::parse();
    let db_conn = sqlx::PgPool::connect(&settings.database_url).await?;
    sqlx::migrate!("../migrations").run(&db_conn).await?;

    // Connect to the Ethereum node
    let web3 = web3::Web3::new(web3::transports::Http::new(&settings.node_url)?);

    let from_block = settings.from_block;
    let to_block = match settings.to_block {
        Some(block) => block,
        None => web3.eth().block_number().await?.as_u64(),
    };

    for block in from_block..=to_block {
        // Retrieve the block with transactions
        let block_with_txs = web3
            .eth()
            .block_with_txs(BlockId::Number(block.into()))
            .await?;

        if let Some(block) = block_with_txs {
            for tx in block.transactions {
                println!("{:?}", tx);
                if tx.input.0.starts_with(TRANSFER_SIGNATURE) && tx.input.0.len() == 68 {
                    let transfer = erc20::Method::Transfer(erc20::transfer::Transfer {
                        hash: tx.hash,
                        from: tx.from.unwrap(),
                        to: H160::from_slice(&tx.input.0[16..36]),
                        value: U256::from(&tx.input.0[36..68]),
                    });
                    println!("{:?}", transfer);

                    if let Some(to) = tx.to {
                        let erc20 = erc20::ERC20::new(to, transfer);
                        erc20.insert(&db_conn).await?;
                    }
                }
            }
        }
    }

    Ok(())
}
