use actix_web::HttpResponse;
use actix_web::{web, Responder};
use serde_json::json;
use sqlx::{PgPool, Row};

/// Returns the number of blocks in the database
pub async fn count(pool: web::Data<PgPool>) -> impl Responder {
    let row = sqlx::query("SELECT COUNT(*) FROM public.block")
        .fetch_one(pool.as_ref())
        .await;

    match row {
        Ok(row) => {
            let count: i64 = row.get(0);
            HttpResponse::Ok().json(json!({ "count": count }))
        }
        Err(err) => {
            eprintln!("Error: {}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}
