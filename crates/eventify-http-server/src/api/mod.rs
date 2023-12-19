pub mod block;
pub mod health;
pub mod log;
pub mod transaction;

pub use health::health;

use actix_web::{web, HttpResponse};
use sqlx::{PgPool, Row};

use crate::types::{CountResponse, ErrorResponse};

pub async fn get_count<'a>(
    conn: web::Data<PgPool>,
    table_name: &'a str,
    description: &'a str,
) -> Result<HttpResponse, HttpResponse> {
    let sql_query = format!("SELECT COUNT(*) FROM eth.{}", table_name);
    let result = sqlx::query(&sql_query).fetch_one(conn.as_ref()).await;

    match result {
        Ok(row) => match row.try_get::<i64, _>(0) {
            Ok(count) => Ok(HttpResponse::Ok().json(CountResponse { count })),
            Err(_) => {
                eprintln!("Error: Failed to parse count");
                Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "Failed to parse count".to_string(),
                }))
            }
        },
        Err(err) => {
            eprintln!("Error: {}", err);
            Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                error: description.to_string(),
            }))
        }
    }
}
