use actix_web::{get, web, Responder};
use sqlx::PgPool;

use crate::api::get_count;

/// Get the Count of Transactions
///
/// This endpoint returns the total count of transactions present in the database.
/// The response is a JSON object containing the count.
///
/// # Responses
///
/// * `200 OK`: Successfully retrieved the count of transactions. The response body will be a JSON object with the structure `{ "count": i64 }`, where `i64` is the total number of transactions.
/// * `500 Internal Server Error`: Indicates that an error occurred on the server while processing the request. The response body will contain a JSON object with an error message.
///
/// # Example
///
/// ```json
/// {
///   "count": 123
/// }
/// ```
#[utoipa::path(
    get,
    path = "/api/v1/transactions/count",
    responses(
        (status = 200, description = "Successfully retrieved the transaction count"),
        (status = 500, description = "Internal Server Error")
    )
)]
#[get("/count")]
pub(crate) async fn get_transactions_count(conn: web::Data<PgPool>) -> impl Responder {
    match get_count(conn, "transaction", "Internal server error").await {
        Ok(response) => response,
        Err(response) => response,
    }
}
