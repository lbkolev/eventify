pub mod erc20;



use actix_web::{web, HttpResponse, Responder};
use serde_derive::{Deserialize, Serialize};

use sqlx::{FromRow, PgPool, Row};


#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct TXCount {
    #[sqlx(flatten)]
    erc20: ERC20,
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct ERC20 {
    #[sqlx(rename = "erc20_approve")]
    pub approve: i64,
    #[sqlx(rename = "erc20_transfer")]
    pub transfer: i64,
    #[sqlx(rename = "erc20_transfer_from")]
    pub transfer_from: i64,
}

pub async fn count(conn: web::Data<PgPool>) -> impl Responder {
    let sql = "SELECT
        (SELECT COUNT(*) FROM contract_fn.approve) as erc20_approve,
        (SELECT COUNT(*) FROM contract_fn.transfer) as erc20_transfer,
        (SELECT COUNT(*) FROM contract_fn.transfer_from) as erc20_transfer_from;
    ";

    let result: Result<TXCount, sqlx::Error> = sqlx::query_as(sql).fetch_one(conn.as_ref()).await;
    match result {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(err) => {
            eprintln!("Error: {}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}
