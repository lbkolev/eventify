use actix_web::HttpResponse;
use actix_web::{web, Responder};
use serde_json::json;
use sqlx::PgPool;

use chainthru_types::IndexedBlock;

/// Returns the number of blocks in the database
pub async fn count(pool: web::Data<PgPool>) -> impl Responder {
    let sql = "SELECT COUNT(*) FROM public.block";
    let row: Result<IndexedBlock, sqlx::Error> = sqlx::query_as(sql).fetch_one(pool.as_ref()).await;

    match row {
        Ok(row) => HttpResponse::Ok().json(json!(row)),
        Err(err) => {
            eprintln!("Error: {}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn number(path: web::Path<u32>, pool: web::Data<PgPool>) -> impl Responder {
    let number = path.into_inner();
    let sql = "SELECT * FROM public.block WHERE number = $1";
    let result: Result<IndexedBlock, sqlx::Error> = sqlx::query_as(sql)
        .bind(number as i64)
        .fetch_one(pool.as_ref())
        .await;

    match result {
        Ok(block) => HttpResponse::Ok().json(json!(block)),
        Err(err) => {
            eprintln!("Error: {}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[cfg(test)]
mod tests {}
