use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use uuid::Uuid;

pub async fn setup_test_db() -> std::result::Result<(Pool<Postgres>, String), sqlx::Error> {
    dotenvy::dotenv().ok();
    let db_url = "postgres://postgres:password@localhost:5432/";
    let master_pool = PgPoolOptions::new()
        .connect(&format!("{}postgres", db_url))
        .await?;

    let db_name = format!("test_{}", Uuid::new_v4().simple());
    let db_url = format!("{}{}", db_url, db_name);

    sqlx::query(&format!("CREATE DATABASE {}", db_name))
        .execute(&master_pool)
        .await?;

    let pool = PgPoolOptions::new().connect(&db_url).await?;

    sqlx::migrate!("../../migrations").run(&pool).await?;

    Ok((pool, db_name))
}

pub async fn teardown_test_db(
    pool: Pool<Postgres>,
    db_name: &str,
) -> std::result::Result<(), sqlx::Error> {
    // Disconnect all connections from the pool
    drop(pool);

    let database_url = "postgres://postgres:password@localhost:5432/postgres";
    let master_pool = PgPoolOptions::new().connect(database_url).await?;

    sqlx::query(&format!("DROP DATABASE IF EXISTS {}", db_name))
        .execute(&master_pool)
        .await?;

    Ok(())
}
