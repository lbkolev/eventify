use std::net::TcpListener;

use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer};
use thiserror::Error;

pub mod api;

use api::block;

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
                        .service(
                            web::scope("/blocks")
                                .route("/count", web::get().to(block::count))
                                .route("/hash/{hash}", web::get().to(HttpResponse::NotImplemented))
                                .route("/number/{number}", web::get().to(block::number)),
                        )
                        .service(
                            web::scope("/transactions")
                                .route("/count", web::get().to(HttpResponse::NotImplemented))
                                .route("/erc20", web::get().to(HttpResponse::NotImplemented)),
                        ),
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
