use actix_web::HttpResponse;
use actix_web::{web, Responder};
use serde_json::json;
use sqlx::{Decode, FromRow, PgPool, Row};
use web3::types::H256;

// struct Block(web3::types::Block<H256>);

#[derive(Debug)]
struct Block {
    hash: H256,
}

/*
impl FromRow<'_, sqlx::postgres::PgRow> for Block {
    fn from_row(row: &sqlx::postgres::PgRow) -> Result<Self, sqlx::Error> {
        Ok(Block(web3::types::Block {
            hash: row.get("hash"),
            parent_hash: row.get("parent_hash"),
            uncles_hash: row.get("uncles_hash"),

            number: row.get("number"),
        }))
    }
}
*/

/*
#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct Block {
    number: i64,
}
*/

impl FromRow<'_, sqlx::postgres::PgRow> for Block {
    fn from_row(row: &sqlx::postgres::PgRow) -> Result<Self, sqlx::Error> {
        Ok(Block {
            hash: row.get("hash"),
        })
    }
}

/*
impl<'de> Decode<'de, DB> for H256 {
    fn decode(value: PgValueRef<'de>) -> Result<Self, Box<dyn Error + 'static + Send + Sync>> {
        // Here, you define how to convert `value` into `H256`.
        // Since H256 is a 32-byte fixed-size array, you can extract the raw bytes and convert them to H256.
        let bytes = value.as_bytes()?;
        if bytes.len() != 32 {
            return Err("Invalid byte length for H256".into());
        }
        let mut array = [0u8; 32];
        array.copy_from_slice(bytes);
        Ok(H256(array))
    }
}
*/

/// Returns the number of blocks in the database
pub async fn count(pool: web::Data<PgPool>) -> impl Responder {
    let sql = "SELECT COUNT(*) FROM public.block";
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

pub async fn hash(path: web::Path<u32>, pool: web::Data<PgPool>) -> impl Responder {
    let hash = path.into_inner();
    let sql = "SELECT * FROM public.block WHERE hash = $1";
    let row = sqlx::query(sql)
        .bind(hash as i64)
        .fetch_one(pool.as_ref())
        .await;

    match row {
        Ok(row) => {
            let count: i64 = row.get(0);
            HttpResponse::Ok().json(json!({ "": count }))
        }
        Err(err) => {
            eprintln!("Error: {}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn number(path: web::Path<u32>, pool: web::Data<PgPool>) -> impl Responder {
    let number = path.into_inner();
    let sql = "SELECT hash FROM public.block WHERE number = $1";
    let row: Result<Block, sqlx::Error> = sqlx::query_as(sql)
        .bind(number as i64)
        .fetch_one(pool.as_ref())
        .await;

    match row {
        Ok(row) => {
            // let count: i64 = row.get(0);
            HttpResponse::Ok().json(json!({ "": row.number }))
        }
        Err(err) => {
            eprintln!("Error: {}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}
