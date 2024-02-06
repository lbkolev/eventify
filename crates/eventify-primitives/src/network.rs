pub mod block;
pub mod contract;
pub mod log;
pub mod transaction;

pub use block::EthBlock;
pub use contract::Contract;
pub use log::{Criteria, EthLog};
pub use transaction::{EthTransaction, TransactionResponse};

pub trait Block {}
impl Block for EthBlock<alloy_primitives::B256> {}
impl Block for EthBlock<EthTransaction> {}

pub trait Transaction {}
impl Transaction for EthTransaction {}

pub trait Log {}
impl Log for EthLog {}

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
            NetworkKind::Ethereum => write!(f, "ethereum"),
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

    Transfer,
    Approval,
    ApprovalForAll,
    Sent,
    Minted,
    Burned,
    AuthorizedOperator,
    RevokedOperator,
    TransferSingle,
    TransferBatch,
    URI,
    Deposit,
    Withdraw,
}

impl std::fmt::Display for LogKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogKind::Raw => write!(f, "log"),
            LogKind::Transfer => write!(f, "log_transfer"),
            LogKind::Approval => write!(f, "log_approval"),
            LogKind::ApprovalForAll => write!(f, "log_approval_for_all"),
            LogKind::Sent => write!(f, "log_sent"),
            LogKind::Minted => write!(f, "log_minted"),
            LogKind::Burned => write!(f, "log_burned"),
            LogKind::AuthorizedOperator => write!(f, "log_authorized_operator"),
            LogKind::RevokedOperator => write!(f, "log_revoked_operator"),
            LogKind::TransferSingle => write!(f, "log_transfer_single"),
            LogKind::TransferBatch => write!(f, "log_transfer_batch"),
            LogKind::URI => write!(f, "log_uri"),
            LogKind::Deposit => write!(f, "log_deposit"),
            LogKind::Withdraw => write!(f, "log_withdraw"),
        }
    }
}
