use actix_web::HttpResponse;
use actix_web::{web, Responder};
use core::num;
use serde_json::json;
use sqlx::sqlx_macros;
use sqlx::{Decode, FromRow, PgPool, Row};
use std::error::Error;
use web3::types::H256;

// struct Block(web3::types::Block<H256>);

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Block {
    #[serde(skip_serializing_if = "Option::is_none")]
    hash: Option<H256>,

    #[serde(skip_serializing_if = "Option::is_none")]
    number: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    parent_hash: Option<[u8; 32]>,
    /*
    uncles_hash: [u8; 32],
    author: [u8; 32],
    state_root: [u8; 32],
    transactions_root: [u8; 32],
    receipts_root: [u8; 32],
    number: i64,
    gas_used: i64,
    gas_limit: i64,
    base_fee_per_gas: i64,
    time_limit: i64,
    difficulty: i64,
    total_difficulty: i64,
    transactions: i64,
    size: i64,
    nonce: [u8; 32],
    */
}

/*
#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct H256Wrapper(H256);

impl<'de> Decode<'de, sqlx::Postgres> for H256Wrapper {
    fn decode(
        value: sqlx::postgres::PgValueRef<'de>,
    ) -> Result<Self, Box<dyn Error + 'static + Send + Sync>> {
        let bytes = value.as_bytes()?;
        if bytes.len() != 32 {
            return Err("Invalid byte length for H256".into());
        }
        let mut array = [0u8; 32];
        array.copy_from_slice(bytes);

        Ok(H256Wrapper(H256::from(array)))
    }
}
*/

impl FromRow<'_, sqlx::postgres::PgRow> for Block {
    fn from_row(row: &sqlx::postgres::PgRow) -> Result<Self, sqlx::Error> {
        let hash = row.try_get("hash").ok().map(H256::from_slice);

        Ok(Block {
            hash,
            number: row.try_get("number").ok(),
            parent_hash: row.try_get("parent_hash").ok(),
            /*
            uncles_hash: row.get("uncles_hash"),
            author: row.get("author"),
            state_root: row.get("state_root"),
            transactions_root: row.get("transactions_root"),
            receipts_root: row.get("receipts_root"),
            number: row.get("number"),
            gas_used: row.get("gas_used"),
            gas_limit: row.get("gas_limit"),
            base_fee_per_gas: row.get("base_fee_per_gas"),
            time_limit: row.get("time_limit"),
            difficulty: row.get("difficulty"),
            total_difficulty: row.get("total_difficulty"),
            transactions: row.get("transactions"),
            size: row.get("size"),
            nonce: row.get("nonce"),
            */
        })
    }
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


impl FromRow<'_, sqlx::postgres::PgRow> for Block {
    fn from_row(row: &sqlx::postgres::PgRow) -> Result<Self, sqlx::Error> {
        Ok(Block {
            hash: row.get("hash"),
        })
    }
}

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
    let sql = "SELECT number FROM public.block WHERE number = $1";
    let res: Result<Block, sqlx::Error> = sqlx::query_as(sql)
        .bind(number as i64)
        .fetch_one(pool.as_ref())
        .await;

    let resp = res.unwrap();
    HttpResponse::Ok().json(json!(resp))
    /*
    match row {
        Ok(row) => {
            //let hash = row.column("hash")
            HttpResponse::Ok().json(json!({ "": row.get::<i64, _>(number as i64) }))
        }
        Err(err) => {
            eprintln!("Error: {}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
    */
}
