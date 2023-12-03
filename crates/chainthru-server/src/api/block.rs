use actix_web::{get, web, HttpResponse, Responder};
use serde_derive::{Deserialize, Serialize};

use sqlx::{prelude::FromRow, PgPool, Row};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct CountResponse {
    count: i64,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct ErrorResponse {
    error: String,
}

/// Get the Count of Blocks
///
/// This endpoint returns the total count of blocks present in the database.
/// The response is a JSON object containing the count.
///
/// # Responses
///
/// * `200 OK`: Successfully retrieved the count of blocks. The response body will be a JSON object with the structure `{ "count": i64 }`, where `i64` is the total number of blocks.
/// * `500 Internal Server Error`: Indicates that an error occurred on the server while processing the request. The response body will contain a JSON object with an error message.
///
/// # Example
///
/// ```json
/// {
///   "count": 42
/// }
/// ```
#[utoipa::path(
    get,
    path = "/api/v1/blocks/count",
    responses(
        (status = 200, description = "Successfully retrieved the block count", body = CountResponse),
        (status = 500, description = "Internal Server Error", body = ErrorResponse)
    )
)]
#[get("/count")]
pub async fn get_blocks_count(conn: web::Data<PgPool>) -> impl Responder {
    let sql = "SELECT COUNT(*) FROM public.block";
    let result = sqlx::query(sql).fetch_one(conn.as_ref()).await;

    match result {
        Ok(row) => match row.try_get::<i64, _>(0) {
            Ok(count) => HttpResponse::Ok().json(CountResponse { count }),
            Err(_) => {
                eprintln!("Error: Failed to parse count");
                HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "Failed to parse count".to_string(),
                })
            }
        },
        Err(err) => {
            eprintln!("Error: {}", err);
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Internal server error".to_string(),
            })
        }
    }
}
