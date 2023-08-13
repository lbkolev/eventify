use std::net::TcpListener;

use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer};

use sqlx::Connection;
use sqlx::{migrate, PgConnection};
use sqlx::{postgres::PgPoolOptions, PgPool};

use crate::api::transaction;
use crate::{
    api::{self, block},
    Result,
};
use chainthru_primitives::DatabaseSettings;

pub struct Application {
    port: u16,
    server: Server,
}

pub async fn startup_migration(settings: DatabaseSettings) -> Result<()> {
    /*
    let tmp: String = settings.into();
    let mut conn = PgConnection::connect(tmp.as_str()).await?;
    migrate!("../migrations").run(conn).await?;
    */
    Ok(())
}

impl Application {
    pub async fn build(settings: crate::Settings) -> Result<Self> {
        let connection_pool = get_connection_pool(&settings.database);
        migrate!("../migrations").run(&connection_pool).await?;

        let file = tokio::fs::File::open("./migrations/data/function_signatures.csv").await?;
        let dbconn: String = settings.database.into();
        let mut conn: PgConnection = PgConnection::connect(&dbconn).await?;

        let res = sqlx::query("SELECT * FROM function_signature")
            .execute(&mut conn)
            .await;
        match res {
            Ok(r) => log::warn!("{:?}", r),
            Err(e) => log::warn!("Error checking function signatures: {}", e),
        }

        let mut copy_in = conn
            .copy_in_raw(r#"COPY function_signature (hex_sig, text_sig) FROM STDIN (FORMAT CSV)"#)
            .await?;
        copy_in.read_from(file).await?;
        match copy_in.finish().await {
            Ok(_) => log::warn!("Successfully imported function signatures"),
            Err(e) => log::warn!("Error importing function signatures: {}", e),
        }

        let listener = TcpListener::bind(format!(
            "{}:{}",
            settings.application.host, settings.application.port
        ))?;
        let port = listener.local_addr().unwrap().port();
        let server = start(
            listener,
            settings.application.worker_threads,
            connection_pool,
        )?;

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> std::result::Result<(), std::io::Error> {
        self.server.await
    }
}

pub fn get_connection_pool(configuration: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        // The connection needs to be established lazily, since the database
        // might not be available when the application starts.
        .connect_lazy_with(configuration.with_db())
}

pub fn start(
    listener: TcpListener,
    worker_threads: usize,
    db_pool: PgPool,
) -> std::result::Result<Server, std::io::Error> {
    let db_pool = web::Data::new(db_pool);

    let server = HttpServer::new(move || {
        App::new()
            .route("/health", web::get().to(api::health))
            .service(
                web::scope("/api").service(
                    web::scope("/v1")
                        .service(
                            web::scope("/blocks")
                                .route("/count", web::get().to(block::count))
                                .route("/hash/{hash}", web::get().to(HttpResponse::NotImplemented))
                                .route("/number/{number}", web::get().to(block::number)),
                        )
                        .service(
                            web::scope("/transactions")
                                .route("/count", web::get().to(transaction::count))
                                .route("/erc20", web::get().to(HttpResponse::NotImplemented))
                                .route("/erc721", web::get().to(HttpResponse::NotImplemented)),
                        ),
                ),
            )
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .workers(worker_threads)
    .run();

    Ok(server)
}
