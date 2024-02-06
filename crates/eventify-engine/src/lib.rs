#![allow(dead_code)]
use eventify_primitives::{networks::NetworkKind, platform::PlatformKind};

use sqlx::{FromRow, Pool, Postgres};

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
        SELECT t.id, t.name, t.network_id, t.platform_id, t.trigger_id, tr.type AS trigger_type, t.channel, t.message
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
            .await
            .unwrap();

        Ok(notifications)
    }

    fn notify(&self) {
        todo!()
    }
}

#[derive(Debug, FromRow)]
struct Notification {
    pub id: i32,
    pub name: String,
    pub network_id: i32,
    pub platform_id: i32,
    pub trigger_id: i32,
    pub trigger_type: TriggerKind,
    pub channel: String,
    pub message: String,
}

pub trait PlatformTrait<T> {
    type Network;
    type Notification;

    fn network(&self) -> Self::Network;
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::postgres::PgPoolOptions;

    #[tokio::test]
    async fn test_engine_fetch_triggers() {
        let database_url = String::from("postgres://postgres:password@localhost:5432/eventify");

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await
            .unwrap();

        let query = r#"
            SELECT t.id, t.name, t.network_id, t.platform_id, t.trigger_id, tr.type AS trigger_type, t.channel, t.message
                FROM public.notification AS t
                    JOIN public.network AS n ON t.network_id = n.id
                    JOIN public.platform AS p ON t.platform_id = p.id
                    JOIN public.trigger AS tr ON t.trigger_id = tr.id
                WHERE n.type = $1
                    AND p.type = $2
                    AND tr.type = $3
        "#;

        let triggers: Vec<Notification> = sqlx::query_as(query)
            .bind(NetworkKind::Ethereum)
            .bind(PlatformKind::Discord)
            .bind(TriggerKind::EachBlock)
            .fetch_all(&pool)
            .await
            .unwrap();

        for trigger in triggers {
            println!("{:?}", trigger);
        }
    }
}
