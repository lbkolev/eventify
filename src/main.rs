use clap::Parser;
use web3::types::{BlockId, BlockNumber};

use chainthru::settings::Settings;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = Settings::parse();
    let db_conn = sqlx::PgPool::connect(&settings.database_url).await?;
    sqlx::migrate!("./migrations").run(&db_conn).await?;

    // Connect to the Ethereum node
    let web3 = web3::Web3::new(web3::transports::Http::new(&settings.node_url)?);

    // Specify the block number or block hash to retrieve
    let block_number = BlockNumber::Number(13950340.into()); // Replace with your desired block number

    // Retrieve the block with transactions
    let block_with_txs = web3
        .eth()
        .block_with_txs(BlockId::Number(block_number))
        .await?;

    //println!("Block: {:#?}", block_with_txs);

    if let Some(block) = block_with_txs {
        for tx in block.transactions {
            println!("Tx: {:0.2X?}", tx);
            if let Some(input) = tx.input.0.strip_prefix(&[0xa9, 0x05, 0x9c, 0xbb]) {
                println!("Input: {:0.2X?}", input);
                let mut input = input.to_vec();
                input.reverse();
                let mut input = input.as_slice();
                let mut amount = [0u8; 32];
                input.read_exact(&mut amount)?;
                let mut amount = amount.as_ref();
                let mut amount = u128::from_le_bytes(amount.try_into().unwrap());
                println!("Amount: {}", amount);
                let mut address = [0u8; 20];
                input.read_exact(&mut address)?;
                let mut address = address.as_ref();
                let mut address = [0u8; 32];
                address.copy_from_slice(&address);
                println!("Address: {:0.2X?}", address);
            }
        }
    }
    Ok(())
}
