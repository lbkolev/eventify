#![allow(dead_code)]
use alloy_primitives::B256;
use eyre::Result;
use redis::Commands;
use sqlx::{FromRow, PgPool, Pool, Postgres};

use tracing::{info, trace};

use eventify_primitives::{
    networks::{ethereum::EthBlock, NetworkKind},
    platform::PlatformKind,
};

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
)]
#[sqlx(type_name = "trigger_type", rename_all = "lowercase")]
pub enum TriggerKind {
    #[sqlx(rename = "each_block")]
    #[default]
    EachBlock,

    #[sqlx(rename = "erc20_transfer")]
    ERC20Transfer,
}

#[derive(Clone, Debug)]
pub struct Discord {
    pub token: String,

    pool: Pool<Postgres>,
}

impl Discord {
    pub async fn new(token: String, pool: Pool<Postgres>) -> Self {
        Self { token, pool }
    }
}

pub trait Notify<T> {
    fn fetch_all(&self) -> impl futures::Future<Output = eyre::Result<Vec<Notification>>>;
    fn notify(&self);
}

impl Notify<Notification> for Discord {
    async fn fetch_all(&self) -> eyre::Result<Vec<Notification>> {
        let query = r#"
        SELECT t.id, t.network_id, t.platform_id, t.trigger_id, tr.type AS trigger_type, t.webhook_url
            FROM public.notification AS t
                JOIN public.network AS n ON t.network_id = n.id
                JOIN public.platform AS p ON t.platform_id = p.id
                JOIN public.trigger AS tr ON t.trigger_id = tr.id
            WHERE n.type = $1
                AND p.type = $2
                AND tr.type = $3
    "#;

        let notifications: Vec<Notification> = sqlx::query_as(query)
            .bind(NetworkKind::Ethereum)
            .bind(PlatformKind::Discord)
            .bind(TriggerKind::EachBlock)
            .fetch_all(&self.pool)
            .await?;

        Ok(notifications)
    }

    fn notify(&self) {
        todo!()
    }
}

#[derive(Debug, FromRow)]
pub struct Notification {
    pub id: i32,
    pub network_id: i32,
    pub platform_id: i32,
    pub trigger_id: i32,
    pub trigger_type: TriggerKind,
    pub webhook_url: String,
}

pub trait PlatformTrait<T> {
    type Network;
    type Notification;

    fn network(&self) -> Self::Network;
}

pub async fn notify(url: String, pool: PgPool) -> Result<()> {
    let mut redis = redis::Client::open(url)?;
    info!("Spawning notify()");

    loop {
        let item: Option<(String, String)> = redis.blpop("eth:block", 125.0)?;

        trace!(item=?item);
        let block: EthBlock<B256> = match item {
            Some((_queue, element)) => serde_json::from_str(&element)?,
            _ => {
                panic!("No block found in the queue!")
            }
        };

        let query = r#"
        SELECT t.id, t.network_id, t.platform_id, t.trigger_id, tr.type AS trigger_type, t.webhook_url
            FROM public.notification AS t
                JOIN public.network AS n ON t.network_id = n.id
                JOIN public.platform AS p ON t.platform_id = p.id
                JOIN public.trigger AS tr ON t.trigger_id = tr.id
            WHERE webhook_url != ''
                AND n.type = $1
                AND p.type = $2
                AND tr.type = $3
"#;

        let notifications: Vec<Notification> = sqlx::query_as(query)
            .bind(NetworkKind::Ethereum)
            .bind(PlatformKind::Discord)
            .bind(TriggerKind::EachBlock)
            .fetch_all(&pool)
            .await?;

        for notif in notifications {
            let _ = reqwest::Client::new()
                .post(notif.webhook_url)
                .json(&serde_json::json!({
                    "content": format!("New block: {:?}", block)
                }))
                .send()
                .await;
        }
    }
}
