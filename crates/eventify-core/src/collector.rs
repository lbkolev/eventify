use std::fmt::Debug;

use alloy_primitives::{Address, FixedBytes, U256};
use alloy_sol_types::SolEvent;
#[cfg(feature = "index")]
use sqlx::PgPool;
use tokio::sync::{mpsc, watch};
use tracing::{info, trace, warn};

use crate::{networks::NetworkClient, CollectT, NetworkT};
use eventify_configs::core::CollectorConfig;
#[cfg(feature = "index")]
use eventify_primitives::InsertT;
use eventify_primitives::{
    events::{ERC1155, ERC20, ERC4626, ERC721, ERC777},
    networks::{Logs, Resource},
    BlockT as _, LogT,
};

#[derive(Debug, Clone)]
pub struct Collector<N>
where
    N: NetworkT,
{
    #[allow(unused)]
    config: CollectorConfig,
    node: N,

    #[cfg(feature = "index")]
    pool: PgPool,

    #[cfg(feature = "propagate")]
    queue_rx: mpsc::Sender<Resource<N::Block, N::Log>>,
}

impl<N> Collector<N>
where
    N: NetworkT,
{
    pub async fn new(
        config: CollectorConfig,
        #[cfg(feature = "index")] pool: PgPool,

        #[cfg(feature = "propagate")] queue_rx: mpsc::Sender<Resource<N::Block, N::Log>>,
    ) -> eyre::Result<Self> {
        let client = NetworkClient::new(config.client_url.clone()).await?;
        let node = N::new(client);

        Ok(Self {
            config,
            node,
            #[cfg(feature = "index")]
            pool,
            #[cfg(feature = "propagate")]
            queue_rx,
        })
    }
}

impl<N> CollectT<crate::Error> for Collector<N>
where
    N: NetworkT,
{
    async fn stream_blocks(&self, stop_signal: watch::Receiver<bool>) -> crate::Result<()> {
        let mut stream = self.node.sub_blocks().await?;
        info!(subscribed = true, kind = "blocks");

        while let Some(block) = stream.next().await {
            if stop_signal.has_changed()? {
                break;
            }

            trace!(block=?block);
            let block = match block {
                Ok(block) => serde_json::from_str::<<N as NetworkT>::Block>(block.get())?,
                Err(err) => {
                    warn!(kind="block_error", err=?err);
                    continue;
                }
            };
            info!(kind="block", number=?block.core().number, hash=?block.core().hash);

            #[cfg(feature = "index")]
            {
                block
                    .insert(&self.pool, &self.config.network.to_string(), &None)
                    .await?;
            }

            #[cfg(feature = "propagate")]
            {
                match self.queue_rx.send(Resource::Block(block)).await {
                    Ok(_) => {}
                    Err(err) => {
                        warn!(kind="propagate_error", err=?err);
                    }
                }
            }
        }

        Ok(())
    }

    async fn stream_logs(&self, stop_signal: watch::Receiver<bool>) -> crate::Result<()> {
        let mut stream = self.node.sub_logs().await?;
        info!(subscribed = true, kind = "logs");

        while let Some(log) = stream.next().await {
            if stop_signal.has_changed()? {
                break;
            }

            trace!(log=?log);
            let log = match log {
                Ok(log) => serde_json::from_str::<<N as NetworkT>::Log>(log.get())?,
                Err(err) => {
                    warn!(kind="log_error", err=?err);
                    continue;
                }
            };
            let event = match_events(log.clone());

            #[cfg(feature = "index")]
            {
                event
                    .insert(
                        &self.pool,
                        &self.config.network.to_string(),
                        &log.core().tx_hash,
                    )
                    .await?;
            }

            #[cfg(feature = "propagate")]
            {
                match self.queue_rx.send(Resource::Log(event)).await {
                    Ok(_) => {}
                    Err(err) => {
                        warn!(kind="propagate_error", err=?err);
                    }
                }
            }
        }

        Ok(())
    }
}

pub fn match_events<L: LogT>(log: L) -> Logs<L> {
    let mut topics = log.core().topics.clone();
    let topics = if log.core().topics.len() == 4 {
        &log.core().topics
    } else {
        while topics.len() < 4 {
            topics.push(FixedBytes::repeat_byte(0));
        }
        &topics
    };
    let data = &log.core().data;
    let signature = topics[0];

    match signature {
        // ERC20 Transfer && ERC721 Transfer share signature
        ERC20::Transfer::SIGNATURE_HASH => {
            match (
                (log.core().data.len() == 0),
                (topics[3] != FixedBytes::repeat_byte(0)),
            ) {
                (true, true) => {
                    // ERC721
                    let from = Address::left_padding_from(&topics[1][12..32]);
                    let to = Address::left_padding_from(&topics[2][12..32]);
                    #[allow(non_snake_case)]
                    let tokenId = U256::from_le_slice(&topics[3][..32]);
                    let e = ERC721::Transfer { from, to, tokenId };

                    info!(kind="log_erc721_transfer", tx_hash=?log.core().tx_hash);
                    Logs::ERC721_Transfer(e)
                }
                (false, false) => {
                    if log.core().data.len() != 32 {
                        warn!(kind="log_raw", sig=ERC20::Transfer::SIGNATURE, tx_hash=?log.core().tx_hash, data_len=?log.core().data.len(), data=?log.core().data);
                        return Logs::Raw(log);
                    }

                    // ERC20
                    let from = Address::left_padding_from(&topics[1][12..32]);
                    let to = Address::left_padding_from(&topics[2][12..32]);
                    let value = U256::from_le_slice(&data[..32]);
                    let e = ERC20::Transfer { from, to, value };

                    info!(kind="log_erc20_transfer", tx_hash=?log.core().tx_hash);
                    Logs::ERC20_Transfer(e)
                }
                _ => {
                    warn!(kind="log_raw", sig=ERC20::Transfer::SIGNATURE, tx_hash=?log.core().tx_hash, data_len=?log.core().data.len(), data=?log.core().data);
                    Logs::Raw(log)
                }
            }
        }

        // ERC20 Approval && ERC721 Approval share signature
        ERC20::Approval::SIGNATURE_HASH => {
            match (
                (log.core().data.len() == 0),
                (topics[3] != FixedBytes::repeat_byte(0)),
            ) {
                (true, true) => {
                    // ERC721
                    let owner = Address::left_padding_from(&topics[1][12..32]);
                    let approved = Address::left_padding_from(&topics[2][12..32]);
                    #[allow(non_snake_case)]
                    let tokenId = U256::from_le_slice(&topics[3][..32]);
                    let e = ERC721::Approval {
                        owner,
                        approved,
                        tokenId,
                    };

                    info!(kind="log_erc721_approval", tx_hash=?log.core().tx_hash);
                    Logs::ERC721_Approval(e)
                }
                (false, false) => {
                    // ERC20
                    if log.core().data.len() != 32 {
                        warn!(kind="log_raw", sig=ERC20::Approval::SIGNATURE, tx_hash=?log.core().tx_hash, data_len=?log.core().data.len(), data=?log.core().data);
                        return Logs::Raw(log);
                    }

                    let owner = Address::left_padding_from(&topics[1][12..32]);
                    let spender = Address::left_padding_from(&topics[2][12..32]);
                    let value = U256::from_le_slice(&data[..32]);
                    let e = ERC20::Approval {
                        owner,
                        spender,
                        value,
                    };

                    info!(kind="log_erc20_approval" , tx_hash=?log.core().tx_hash);
                    Logs::ERC20_Approval(e)
                }
                _ => {
                    warn!(kind="log_raw", sig=ERC20::Approval::SIGNATURE, tx_hash=?log.core().tx_hash, data_len=?log.core().data.len(), data=?log.core().data);
                    Logs::Raw(log)
                }
            }
        }

        ERC721::ApprovalForAll::SIGNATURE_HASH => {
            if log.core().data.len() != 32 {
                warn!(kind="log_raw", sig=ERC721::ApprovalForAll::SIGNATURE, tx_hash=?log.core().tx_hash, data_len=?log.core().data.len(), data=?log.core().data);
                return Logs::Raw(log);
            }

            let owner = Address::left_padding_from(&topics[1][12..32]);
            let operator = Address::left_padding_from(&topics[2][12..32]);
            let approved = data.ends_with(&[0x1]);
            let e = ERC721::ApprovalForAll {
                owner,
                operator,
                approved,
            };

            info!(kind="log_erc721_approval_for_all", tx_hash=?log.core().tx_hash);
            Logs::ERC721_ApprovalForAll(e)
        }

        ERC777::Sent::SIGNATURE_HASH => {
            if log.core().data.len() != 96 {
                warn!(kind="log_raw", sig=ERC777::Sent::SIGNATURE, tx_hash=?log.core().tx_hash, data_len=?log.core().data.len(), data=?log.core().data);
                return Logs::Raw(log);
            }

            let operator = Address::left_padding_from(&topics[1][12..32]);
            let from = Address::left_padding_from(&topics[2][12..32]);
            let to = Address::left_padding_from(&topics[3][12..32]);
            let amount = U256::from_le_slice(&log.core().data[..32]);
            let data = log.core().data[32..64].to_vec();
            #[allow(non_snake_case)]
            let operatorData = log.core().data[64..96].to_vec();
            let e = ERC777::Sent {
                operator,
                from,
                to,
                amount,
                data,
                operatorData,
            };

            info!(kind="log_erc777_sent", tx_hash=?log.core().tx_hash);
            Logs::ERC777_Sent(e)
        }

        ERC777::Minted::SIGNATURE_HASH => {
            if log.core().data.len() != 96 {
                warn!(kind="log_raw", sig=ERC777::Minted::SIGNATURE, tx_hash=?log.core().tx_hash, data_len=?log.core().data.len(), data=?log.core().data);
                return Logs::Raw(log);
            }

            let operator = Address::left_padding_from(&topics[1][12..32]);
            let to = Address::left_padding_from(&topics[2][12..32]);
            let amount = U256::from_le_slice(&log.core().data[..32]);
            let data = log.core().data[32..64].to_vec();
            #[allow(non_snake_case)]
            let operatorData = log.core().data[64..96].to_vec();
            let e = ERC777::Minted {
                operator,
                to,
                amount,
                data,
                operatorData,
            };

            info!(kind="log_erc777_minted", tx_hash=?log.core().tx_hash);
            Logs::ERC777_Minted(e)
        }

        ERC777::Burned::SIGNATURE_HASH => {
            if log.core().data.len() != 96 {
                warn!(kind="log_raw", sig=ERC777::Burned::SIGNATURE, tx_hash=?log.core().tx_hash, data_len=?log.core().data.len(), data=?log.core().data);
                return Logs::Raw(log);
            }

            let operator = Address::left_padding_from(&topics[1][12..32]);
            let from = Address::left_padding_from(&topics[2][12..32]);
            let amount = U256::from_le_slice(&log.core().data[..32]);
            let data = log.core().data[32..64].to_vec();
            #[allow(non_snake_case)]
            let operatorData = log.core().data[64..96].to_vec();
            let e = ERC777::Burned {
                operator,
                from,
                amount,
                data,
                operatorData,
            };

            info!(kind="log_erc777_burned", tx_hash=?log.core().tx_hash);
            Logs::ERC777_Burned(e)
        }

        ERC777::AuthorizedOperator::SIGNATURE_HASH => {
            let operator = Address::left_padding_from(&topics[1][12..32]);
            let holder = Address::left_padding_from(&topics[2][12..32]);
            let e = ERC777::AuthorizedOperator { operator, holder };

            info!(kind="log_erc777_authorized_operator", tx_hash=?log.core().tx_hash);
            Logs::ERC777_AuthorizedOperator(e)
        }

        ERC777::RevokedOperator::SIGNATURE_HASH => {
            let operator = Address::left_padding_from(&topics[1][12..32]);
            let holder = Address::left_padding_from(&topics[2][12..32]);
            let e = ERC777::RevokedOperator { operator, holder };

            info!(kind="log_erc777_revoked_operator", tx_hash=?log.core().tx_hash);
            Logs::ERC777_RevokedOperator(e)
        }

        ERC1155::TransferSingle::SIGNATURE_HASH => {
            if log.core().data.len() != 64 {
                warn!(kind="log_raw", sig=ERC1155::TransferSingle::SIGNATURE, tx_hash=?log.core().tx_hash, data_len=?log.core().data.len(), data=?log.core().data);
                return Logs::Raw(log);
            }

            let operator = Address::left_padding_from(&topics[1][12..32]);
            let from = Address::left_padding_from(&topics[2][12..32]);
            let to = Address::left_padding_from(&topics[3][12..32]);
            let id = U256::from_le_slice(&log.core().data[..32]);
            let value = U256::from_le_slice(&log.core().data[32..64]);
            let e = ERC1155::TransferSingle {
                operator,
                from,
                to,
                id,
                value,
            };

            info!(kind="erc1155_transfer_single", tx_hash=?log.core().tx_hash);
            Logs::ERC1155_TransferSingle(e)
        }

        ERC1155::TransferBatch::SIGNATURE_HASH => {
            if log.core().data.len() != 64 {
                warn!(kind="log_raw", sig=ERC1155::TransferBatch::SIGNATURE, tx_hash=?log.core().tx_hash, data_len=?log.core().data.len(), data=?log.core().data);
                return Logs::Raw(log);
            }

            let operator = Address::left_padding_from(&topics[1][12..32]);
            let from = Address::left_padding_from(&topics[2][12..32]);
            let to = Address::left_padding_from(&topics[3][12..32]);
            let ids = log.core().data[..32]
                .chunks_exact(32)
                .map(U256::from_le_slice)
                .collect::<Vec<_>>();
            let values = log.core().data[32..64]
                .chunks_exact(32)
                .map(U256::from_le_slice)
                .collect::<Vec<_>>();
            let e = ERC1155::TransferBatch {
                operator,
                from,
                to,
                ids,
                values,
            };

            info!(kind="erc1155_transfer_batch", tx_hash=?log.core().tx_hash);
            Logs::ERC1155_TransferBatch(e)
        }

        ERC1155::URI::SIGNATURE_HASH => {
            let value = log.core().data.to_string();
            let id = U256::from_le_slice(&topics[1][..]);
            let e = ERC1155::URI { value, id };

            info!(kind="erc1155_uri", tx_hash=?log.core().tx_hash);
            Logs::ERC1155_URI(e)
        }

        ERC4626::Deposit::SIGNATURE_HASH => {
            if log.core().data.len() != 64 {
                warn!(kind="log_raw", sig=ERC4626::Deposit::SIGNATURE, tx_hash=?log.core().tx_hash, data_len=?log.core().data.len(), data=?log.core().data);
                return Logs::Raw(log);
            }

            let sender = Address::left_padding_from(&topics[1][12..32]);
            let owner = Address::left_padding_from(&topics[2][12..32]);
            let assets = U256::from_le_slice(&log.core().data[..32]);
            let shares = U256::from_le_slice(&log.core().data[32..64]);
            let e = ERC4626::Deposit {
                sender,
                owner,
                assets,
                shares,
            };

            info!(kind="erc4626_deposit", tx_hash=?log.core().tx_hash);
            Logs::ERC4626_Deposit(e)
        }

        ERC4626::Withdraw::SIGNATURE_HASH => {
            if log.core().data.len() != 64 {
                warn!(kind="log_raw", sig=ERC4626::Withdraw::SIGNATURE, tx_hash=?log.core().tx_hash, data_len=?log.core().data.len(), data=?log.core().data);
                return Logs::Raw(log);
            }

            let sender = Address::left_padding_from(&topics[1][12..32]);
            let receiver = Address::left_padding_from(&topics[2][12..32]);
            let owner = Address::left_padding_from(&topics[3][12..32]);
            let assets = U256::from_le_slice(&log.core().data[..32]);
            let shares = U256::from_le_slice(&log.core().data[32..64]);
            let e = ERC4626::Withdraw {
                sender,
                receiver,
                owner,
                assets,
                shares,
            };

            info!(kind="erc4626_withdraw", tx_hash=?log.core().tx_hash);
            Logs::ERC4626_Withdraw(e)
        }

        _ => {
            info!(kind="log_raw", address=?log.core().address, tx_hash=?log.core().tx_hash);
            Logs::Raw(log)
        }
    }
}
