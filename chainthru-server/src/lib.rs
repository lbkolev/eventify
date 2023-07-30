use std::net::TcpListener;

use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer};
use thiserror::Error;

pub mod api;

use api::block;
use api::generic;

pub async fn run(settings: AppSettings) -> std::result::Result<Server, crate::Error> {
    let listener = TcpListener::bind(format!("{}:{}", settings.host, settings.port))?;
    let db_pool = sqlx::PgPool::connect(&settings.database_url).await?;
    let conn = web::Data::new(db_pool);

    let server = HttpServer::new(move || {
        App::new()
            .route("/health", web::get().to(api::health))
            .service(
                web::scope("/api").service(
                    web::scope("/v1")
                        .route(
                            "/count/{name}/{type}/{method}",
                            web::get().to(api::generic::count),
                        )
                        .route("/count/{name}", web::get().to(api::generic::count))
                        .service(web::scope("/block").route("/count", web::get().to(block::count)))
                        .service(
                            web::scope("/transaction")
                                .route("/erc20", web::get().to(HttpResponse::Ok))
                                .route("/tmp", web::get().to(api::transaction::erc20::test2)),
                        )
                        .route("/placeholder", web::post().to(HttpResponse::Ok)),
                ),
            )
            .app_data(conn.clone())
    })
    .listen(listener)?
    .workers(settings.worker_threads)
    .run();

    Ok(server)
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to bind address")]
    BindAddress(#[from] std::io::Error),

    #[error("Failed to connect to database")]
    ConnectToDatabase(#[from] sqlx::Error),
}

#[derive(Clone, Debug)]
pub struct AppSettings {
    pub port: u16,
    pub host: String,
    pub database_url: String,

    /// The number of workers to start
    ///
    /// by default, the number of the machine's physical cores.
    pub worker_threads: usize,
}
