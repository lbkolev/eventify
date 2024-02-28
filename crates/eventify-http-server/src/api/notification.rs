use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::{FromRow, PgPool};

use eventify_engine::TriggerKind;
use eventify_primitives::{networks::NetworkKind, platform::PlatformKind};

#[derive(Debug, FromRow, Deserialize)]
pub struct NotificationData {
    network: NetworkKind,
    platform: PlatformKind,
    trigger: TriggerKind,
    webhook_url: String,
}

#[post("/")]
pub(crate) async fn set_notification(
    conn: web::Data<PgPool>,
    payload: web::Form<NotificationData>,
) -> impl Responder {
    let query = r#"
      INSERT INTO public.notification (network_id, platform_id, trigger_id, webhook_url) VALUES (
        (SELECT id FROM network WHERE type=$1 LIMIT 1),
        (SELECT id FROM platform WHERE type=$2 LIMIT 1),
        (SELECT id FROM trigger WHERE type=$3 LIMIT 1),
        $4
      )
    "#;

    let result = sqlx::query(query)
        .bind(payload.network)
        .bind(payload.platform)
        .bind(payload.trigger)
        .bind(payload.webhook_url.clone())
        .execute(conn.get_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
