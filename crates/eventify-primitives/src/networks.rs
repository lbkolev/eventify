pub mod ethereum;

use std::fmt::Display;

use alloy_primitives::B256;
use sqlx::PgPool;

use crate::{
    events::{ERC1155, ERC20, ERC4626, ERC721, ERC777},
    BlockT, EmitT, InsertT, LogT, TransactionT,
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
#[sqlx(type_name = "network_type", rename_all = "lowercase")]
pub enum NetworkKind {
    #[default]
    Ethereum,

    Zksync,
}

impl std::fmt::Display for NetworkKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NetworkKind::Ethereum => write!(f, "eth"),
            NetworkKind::Zksync => write!(f, "zksync"),
        }
    }
}

impl std::str::FromStr for NetworkKind {
    type Err = NetworkKindError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "ethereum" | "eth" => Ok(NetworkKind::Ethereum),
            "zksync" => Ok(NetworkKind::Zksync),
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
    async fn insert(
        &self,
        pool: &PgPool,
        schema: &str,
        tx_hash: &Option<B256>,
    ) -> eyre::Result<(), sqlx::Error> {
        match self {
            Logs::Raw(log) => log.insert(pool, schema, tx_hash).await?,
            Logs::ERC20_Transfer(e) => e.insert(pool, schema, tx_hash).await?,
            Logs::ERC20_Approval(e) => e.insert(pool, schema, tx_hash).await?,

            Logs::ERC721_Transfer(e) => e.insert(pool, schema, tx_hash).await?,
            Logs::ERC721_Approval(e) => e.insert(pool, schema, tx_hash).await?,
            Logs::ERC721_ApprovalForAll(e) => e.insert(pool, schema, tx_hash).await?,

            Logs::ERC777_Sent(e) => e.insert(pool, schema, tx_hash).await?,
            Logs::ERC777_Minted(e) => e.insert(pool, schema, tx_hash).await?,
            Logs::ERC777_Burned(e) => e.insert(pool, schema, tx_hash).await?,
            Logs::ERC777_AuthorizedOperator(e) => e.insert(pool, schema, tx_hash).await?,
            Logs::ERC777_RevokedOperator(e) => e.insert(pool, schema, tx_hash).await?,

            Logs::ERC1155_TransferSingle(e) => e.insert(pool, schema, tx_hash).await?,
            Logs::ERC1155_TransferBatch(e) => e.insert(pool, schema, tx_hash).await?,
            Logs::ERC1155_URI(e) => e.insert(pool, schema, tx_hash).await?,

            Logs::ERC4626_Deposit(e) => e.insert(pool, schema, tx_hash).await?,
            Logs::ERC4626_Withdraw(e) => e.insert(pool, schema, tx_hash).await?,
        }

        Ok(())
    }
}

impl<L: LogT> EmitT for Logs<L> {
    async fn emit<T: serde::Serialize + Send + Sync>(
        &self,
        queue: &redis::Client,
        network: &crate::networks::NetworkKind,
        message: &T,
    ) -> eyre::Result<(), redis::RedisError> {
        match self {
            Logs::Raw(log) => log.emit(queue, network, message).await?,
            Logs::ERC20_Transfer(e) => e.emit(queue, network, message).await?,
            Logs::ERC20_Approval(e) => e.emit(queue, network, message).await?,

            Logs::ERC721_Transfer(e) => e.emit(queue, network, message).await?,
            Logs::ERC721_Approval(e) => e.emit(queue, network, message).await?,
            Logs::ERC721_ApprovalForAll(e) => e.emit(queue, network, message).await?,

            Logs::ERC777_Sent(e) => e.emit(queue, network, message).await?,
            Logs::ERC777_Minted(e) => e.emit(queue, network, message).await?,
            Logs::ERC777_Burned(e) => e.emit(queue, network, message).await?,
            Logs::ERC777_AuthorizedOperator(e) => e.emit(queue, network, message).await?,
            Logs::ERC777_RevokedOperator(e) => e.emit(queue, network, message).await?,

            Logs::ERC1155_TransferSingle(e) => e.emit(queue, network, message).await?,
            Logs::ERC1155_TransferBatch(e) => e.emit(queue, network, message).await?,
            Logs::ERC1155_URI(e) => e.emit(queue, network, message).await?,

            Logs::ERC4626_Deposit(e) => e.emit(queue, network, message).await?,
            Logs::ERC4626_Withdraw(e) => e.emit(queue, network, message).await?,
        }

        Ok(())
    }
}

impl<T: LogT> Display for Logs<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Logs::Raw(_) => write!(f, "log"),
            Logs::ERC20_Transfer(_) => write!(f, "log_erc20_transfer"),
            Logs::ERC20_Approval(_) => write!(f, "log_erc20_approval"),

            Logs::ERC721_Transfer(_) => write!(f, "log_erc721_transfer"),
            Logs::ERC721_Approval(_) => write!(f, "log_erc721_approval"),
            Logs::ERC721_ApprovalForAll(_) => write!(f, "log_erc20_approval_for_all"),

            Logs::ERC777_Sent(_) => write!(f, "log_erc777_sent"),
            Logs::ERC777_Minted(_) => write!(f, "log_erc777_minted"),
            Logs::ERC777_Burned(_) => write!(f, "log_erc777_burned"),
            Logs::ERC777_AuthorizedOperator(_) => write!(f, "log_erc777_authorized_operator"),
            Logs::ERC777_RevokedOperator(_) => write!(f, "log_erc777_revoked_operator"),

            Logs::ERC1155_TransferSingle(_) => write!(f, "log_erc1155_transfer_single"),
            Logs::ERC1155_TransferBatch(_) => write!(f, "log_erc1155_transfer_batch"),
            Logs::ERC1155_URI(_) => write!(f, "log_erc1155_uri"),

            Logs::ERC4626_Deposit(_) => write!(f, "log_erc4626_deposit"),
            Logs::ERC4626_Withdraw(_) => write!(f, "log_erc4626_withdraw"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Resource<B, T, L>
where
    B: BlockT,
    T: TransactionT,
    L: LogT,
{
    Block(B),
    Tx(T),
    Log(Logs<L>),
}

impl<B, T, L> InsertT for Resource<B, T, L>
where
    B: BlockT,
    T: TransactionT,
    L: LogT,
{
    async fn insert(
        &self,
        pool: &PgPool,
        schema: &str,
        tx_hash: &Option<B256>,
    ) -> eyre::Result<(), sqlx::Error> {
        match self {
            Resource::Block(block) => block.insert(pool, schema, tx_hash).await?,
            Resource::Tx(tx) => tx.insert(pool, schema, tx_hash).await?,
            Resource::Log(log) => log.insert(pool, schema, tx_hash).await?,
        }

        Ok(())
    }
}

impl<B, T, L> EmitT for Resource<B, T, L>
where
    B: BlockT,
    T: TransactionT,
    L: LogT,
{
    async fn emit<M: serde::Serialize + Send + Sync>(
        &self,
        queue: &redis::Client,
        network: &crate::networks::NetworkKind,
        message: &M,
    ) -> eyre::Result<(), redis::RedisError> {
        match self {
            Resource::Block(block) => block.emit(queue, network, message).await?,
            Resource::Tx(tx) => tx.emit(queue, network, message).await?,
            Resource::Log(log) => log.emit(queue, network, message).await?,
        }

        Ok(())
    }
}

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize, utoipa::ToSchema,
)]
pub enum ResourceKind {
    Block,
    Transaction,
    Log(LogKind),
}

impl ResourceKind {
    pub fn resources_from_string(s: String) -> std::collections::HashSet<ResourceKind> {
        s.split(',')
            .map(|x| match x.trim().to_lowercase().as_str() {
                "block" | "blocks" => ResourceKind::Block,
                "tx" | "txs" | "transactions" => ResourceKind::Transaction,
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
            ResourceKind::Transaction => write!(f, "tx"),
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
            LogKind::ERC721_ApprovalForAll => write!(f, "log_erc20_approval_for_all"),

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
