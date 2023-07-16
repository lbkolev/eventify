use clap::Parser;
use ethereum_types::{H160, U256};
use web3::types::{BlockId, BlockNumber};

use chainthru::indexer::erc20::{self, TRANSFER_SIGNATURE};
use chainthru::settings::Settings;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = Settings::parse();
    let db_conn = sqlx::PgPool::connect(&settings.database_url).await?;
    sqlx::migrate!("./migrations").run(&db_conn).await?;

    // Connect to the Ethereum node
    let web3 = web3::Web3::new(web3::transports::Http::new(&settings.node_url)?);

    // Specify the block number or block hash to retrieve
    let from_block = BlockNumber::Number(settings.from_block.into()); // Replace with your desired block number
    let to_block = BlockNumber::Number(
        settings
            .to_block
            .unwrap_or_else(|| web3.eth().block_number().await.unwrap().into())
            .into(),
    );

    // Retrieve the block with transactions
    let block_with_txs = web3
        .eth()
        .block_with_txs(BlockId::Number(block_number))
        .await?;

    if let Some(block) = block_with_txs {
        for tx in block.transactions {
            if tx.input.0.starts_with(TRANSFER_SIGNATURE) {
                // println!("Tx: {:0.2X?}", tx);

                let transfer = erc20::Transfer {
                    from: tx.from.unwrap(),
                    to: H160::from_slice(&tx.input.0[16..36]),
                    value: U256::from(&tx.input.0[36..]),
                };

                println!("Transfer: {:?}", transfer);
            }
        }
    }

    Ok(())
}
