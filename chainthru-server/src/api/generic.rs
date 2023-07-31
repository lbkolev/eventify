use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use sqlx::{PgPool, Row};

pub async fn count(
    path: web::Path<(String, String, String)>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    let (name, type_, method) = path.into_inner();

    if !["block", "transaction"].contains(&name.as_str()) {
        return HttpResponse::BadRequest().finish();
    }

    let sql = match name.as_str() {
        "block" => "SELECT COUNT(*) FROM public.block",

        "transaction" => match type_.as_str() {
            "erc20" => match method.as_str() {
                "transfer" => "SELECT COUNT(*) FROM erc20.transfer",

                _ => {
                    "SELECT (
                        SELECT COUNT(*) FROM erc20.transfer
                    ) as transfer,
                    (
                        SELECT COUNT(*) FROM erc20.approval
                    ) as approval
                    FROM all_erc20_transactions"
                }
            },

            _ => {
                "SELECT (
                    SELECT COUNT(*) FROM erc20.transfer
                ) as erc20_transfer,
                (
                    SELECT COUNT(*) FROM erc20.approval
                ) as erc20_approval
                FROM all_transactions
                "
            }
        },

        _ => "SELECT 0",
    };

    let row = sqlx::query(sql).fetch_one(pool.as_ref()).await;
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
