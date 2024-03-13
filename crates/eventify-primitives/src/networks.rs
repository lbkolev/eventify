pub mod arbitrum;
pub mod avalanche;
pub mod base;
pub mod bsc;
pub mod core;
pub mod ethereum;
pub mod linea;
pub mod optimism;
pub mod polygon;
pub mod zksync;

use alloy_primitives::B256;
use sqlx::{Error as SqlError, PgPool};

use crate::{
    events::{ERC1155, ERC20, ERC4626, ERC721, ERC777},
    BlockT, EmitError, EmitT, InsertT, LogT,
};

#[derive(Clone, Debug)]
pub struct NetworkKindError(String);
impl std::error::Error for NetworkKindError {}
impl std::fmt::Display for NetworkKindError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid network: {}", self.0)
    }
}

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    PartialEq,
    Eq,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    sqlx::Type,
    utoipa::ToSchema,
)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "network_type", rename_all = "lowercase")]
pub enum NetworkKind {
    #[default]
    Ethereum,
    Zksync,
    Polygon,
    Optimism,
    Arbitrum,
    Linea,
    Avalanche,
    Bsc,
    Base,
}

impl std::fmt::Display for NetworkKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NetworkKind::Ethereum => write!(f, "eth"),
            NetworkKind::Zksync => write!(f, "zksync"),
            NetworkKind::Polygon => write!(f, "polygon"),
            NetworkKind::Optimism => write!(f, "optimism"),
            NetworkKind::Arbitrum => write!(f, "arbitrum"),
            NetworkKind::Linea => write!(f, "linea"),
            NetworkKind::Avalanche => write!(f, "avalanche"),
            NetworkKind::Bsc => write!(f, "bsc"),
            NetworkKind::Base => write!(f, "base"),
        }
    }
}

impl std::str::FromStr for NetworkKind {
    type Err = NetworkKindError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "ethereum" | "eth" => Ok(NetworkKind::Ethereum),
            "zksync" => Ok(NetworkKind::Zksync),
            "polygon" => Ok(NetworkKind::Polygon),
            "optimism" => Ok(NetworkKind::Optimism),
            "arbitrum" => Ok(NetworkKind::Arbitrum),
            "linea" => Ok(NetworkKind::Linea),
            "avalanche" => Ok(NetworkKind::Avalanche),
            "bsc" => Ok(NetworkKind::Bsc),
            "base" => Ok(NetworkKind::Base),
            _ => Err(NetworkKindError(s.to_string())),
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, serde::Serialize)]
pub enum Logs<L: LogT> {
    Raw(L),

    ERC20_Transfer(ERC20::Transfer),
    ERC20_Approval(ERC20::Approval),

    ERC721_Transfer(ERC721::Transfer),
    ERC721_Approval(ERC721::Approval),
    ERC721_ApprovalForAll(ERC721::ApprovalForAll),

    ERC777_Sent(ERC777::Sent),
    ERC777_Minted(ERC777::Minted),
    ERC777_Burned(ERC777::Burned),
    ERC777_AuthorizedOperator(ERC777::AuthorizedOperator),
    ERC777_RevokedOperator(ERC777::RevokedOperator),

    ERC1155_TransferSingle(ERC1155::TransferSingle),
    ERC1155_TransferBatch(ERC1155::TransferBatch),
    ERC1155_URI(ERC1155::URI),

    ERC4626_Deposit(ERC4626::Deposit),
    ERC4626_Withdraw(ERC4626::Withdraw),
}

impl<L: LogT> InsertT for Logs<L> {
    async fn insert(&self, pool: &PgPool, tx_hash: &Option<B256>) -> eyre::Result<(), SqlError> {
        match self {
            Logs::Raw(log) => log.insert(pool, tx_hash).await?,
            Logs::ERC20_Transfer(e) => e.insert(pool, tx_hash).await?,
            Logs::ERC20_Approval(e) => e.insert(pool, tx_hash).await?,

            Logs::ERC721_Transfer(e) => e.insert(pool, tx_hash).await?,
            Logs::ERC721_Approval(e) => e.insert(pool, tx_hash).await?,
            Logs::ERC721_ApprovalForAll(e) => e.insert(pool, tx_hash).await?,

            Logs::ERC777_Sent(e) => e.insert(pool, tx_hash).await?,
            Logs::ERC777_Minted(e) => e.insert(pool, tx_hash).await?,
            Logs::ERC777_Burned(e) => e.insert(pool, tx_hash).await?,
            Logs::ERC777_AuthorizedOperator(e) => e.insert(pool, tx_hash).await?,
            Logs::ERC777_RevokedOperator(e) => e.insert(pool, tx_hash).await?,

            Logs::ERC1155_TransferSingle(e) => e.insert(pool, tx_hash).await?,
            Logs::ERC1155_TransferBatch(e) => e.insert(pool, tx_hash).await?,
            Logs::ERC1155_URI(e) => e.insert(pool, tx_hash).await?,

            Logs::ERC4626_Deposit(e) => e.insert(pool, tx_hash).await?,
            Logs::ERC4626_Withdraw(e) => e.insert(pool, tx_hash).await?,
        }

        Ok(())
    }
}

impl<L: LogT + serde::Serialize> EmitT for Logs<L> {
    async fn emit(
        &self,
        queue: &redis::Client,
        network: &crate::networks::NetworkKind,
    ) -> eyre::Result<(), EmitError> {
        match self {
            Logs::Raw(log) => log.emit(queue, network).await?,
            Logs::ERC20_Transfer(e) => e.emit(queue, network).await?,
            Logs::ERC20_Approval(e) => e.emit(queue, network).await?,

            Logs::ERC721_Transfer(e) => e.emit(queue, network).await?,
            Logs::ERC721_Approval(e) => e.emit(queue, network).await?,
            Logs::ERC721_ApprovalForAll(e) => e.emit(queue, network).await?,

            Logs::ERC777_Sent(e) => e.emit(queue, network).await?,
            Logs::ERC777_Minted(e) => e.emit(queue, network).await?,
            Logs::ERC777_Burned(e) => e.emit(queue, network).await?,
            Logs::ERC777_AuthorizedOperator(e) => e.emit(queue, network).await?,
            Logs::ERC777_RevokedOperator(e) => e.emit(queue, network).await?,

            Logs::ERC1155_TransferSingle(e) => e.emit(queue, network).await?,
            Logs::ERC1155_TransferBatch(e) => e.emit(queue, network).await?,
            Logs::ERC1155_URI(e) => e.emit(queue, network).await?,

            Logs::ERC4626_Deposit(e) => e.emit(queue, network).await?,
            Logs::ERC4626_Withdraw(e) => e.emit(queue, network).await?,
        }

        Ok(())
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub enum Resource<B, L>
where
    B: BlockT,
    L: LogT,
{
    Block(B),
    Log(Logs<L>),
}

impl<B, L> InsertT for Resource<B, L>
where
    B: BlockT,
    L: LogT,
{
    async fn insert(&self, pool: &PgPool, tx_hash: &Option<B256>) -> eyre::Result<(), SqlError> {
        match self {
            Resource::Block(block) => block.insert(pool, tx_hash).await?,
            Resource::Log(log) => log.insert(pool, tx_hash).await?,
        }

        Ok(())
    }
}

impl<B, L> EmitT for Resource<B, L>
where
    B: BlockT,
    L: LogT,
{
    async fn emit(
        &self,
        queue: &redis::Client,
        network: &crate::networks::NetworkKind,
    ) -> eyre::Result<(), EmitError> {
        match self {
            Resource::Block(block) => block.emit(queue, network).await?,
            Resource::Log(log) => log.emit(queue, network).await?,
        }

        Ok(())
    }
}

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize, utoipa::ToSchema,
)]
#[serde(rename_all = "lowercase")]
pub enum ResourceKind {
    Block,
    Log(LogKind),
}

impl ResourceKind {
    pub fn resources_from_string(s: String) -> std::collections::HashSet<ResourceKind> {
        s.split(',')
            .map(|x| match x.trim().to_lowercase().as_str() {
                "block" | "blocks" => ResourceKind::Block,
                "log" | "logs" => ResourceKind::Log(LogKind::Raw),
                _ => {
                    panic!("invalid resource: {}", x);
                }
            })
            .collect()
    }

    pub fn resources_from_str(s: &str) -> std::collections::HashSet<ResourceKind> {
        ResourceKind::resources_from_string(s.to_string())
    }
}

impl std::fmt::Display for ResourceKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResourceKind::Block => write!(f, "block"),
            ResourceKind::Log(kind) => write!(f, "{}", kind),
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    PartialEq,
    Eq,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    sqlx::Type,
    utoipa::ToSchema,
)]
#[serde(rename_all = "lowercase")]
pub enum LogKind {
    #[default]
    Raw,

    ERC20_Transfer,
    ERC20_Approval,

    ERC721_Transfer,
    ERC721_Approval,
    ERC721_ApprovalForAll,

    ERC777_Sent,
    ERC777_Minted,
    ERC777_Burned,
    ERC777_AuthorizedOperator,
    ERC777_RevokedOperator,

    ERC1155_TransferSingle,
    ERC1155_TransferBatch,
    ERC1155_URI,

    ERC4626_Deposit,
    ERC4626_Withdraw,
}

impl std::fmt::Display for LogKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogKind::Raw => write!(f, "log"),
            LogKind::ERC20_Transfer => write!(f, "log_erc20_transfer"),
            LogKind::ERC20_Approval => write!(f, "log_erc20_approval"),

            LogKind::ERC721_Transfer => write!(f, "log_erc721_transfer"),
            LogKind::ERC721_Approval => write!(f, "log_erc721_approval"),
            LogKind::ERC721_ApprovalForAll => write!(f, "log_erc721_approval_for_all"),

            LogKind::ERC777_Sent => write!(f, "log_erc777_sent"),
            LogKind::ERC777_Minted => write!(f, "log_erc777_minted"),
            LogKind::ERC777_Burned => write!(f, "log_erc777_burned"),
            LogKind::ERC777_AuthorizedOperator => write!(f, "log_erc777_authorized_operator"),
            LogKind::ERC777_RevokedOperator => write!(f, "log_erc777_revoked_operator"),

            LogKind::ERC1155_TransferSingle => write!(f, "log_erc1155_transfer_single"),
            LogKind::ERC1155_TransferBatch => write!(f, "log_erc1155_transfer_batch"),
            LogKind::ERC1155_URI => write!(f, "log_erc1155_uri"),

            LogKind::ERC4626_Deposit => write!(f, "log_erc4626_deposit"),
            LogKind::ERC4626_Withdraw => write!(f, "log_erc4626_withdraw"),
        }
    }
}
