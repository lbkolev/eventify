use clap::Parser;
use ethereum_types::{H160, U256};
use web3::types::BlockId;

use chainthru::indexer::transaction::erc20::{self, TRANSFER_SIGNATURE};
use chainthru::settings::Settings;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = Settings::parse();
    let db_conn = sqlx::PgPool::connect(&settings.database_url).await?;
    sqlx::migrate!("./migrations").run(&db_conn).await?;

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
