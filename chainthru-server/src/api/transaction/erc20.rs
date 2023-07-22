use actix_web::{web, Responder};
use serde_json::json;

pub async fn test2() -> impl Responder {
    web::Json(json!({"status": "ok"}))
}
