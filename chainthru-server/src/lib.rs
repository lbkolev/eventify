use std::net::TcpListener;

use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer};

pub mod api;

pub fn run(
    listener: TcpListener,
    db_pool: sqlx::PgPool,
) -> std::result::Result<Server, std::io::Error> {
    let server = HttpServer::new(move || {
        App::new()
            .route("/health", web::get().to(api::health))
            .service(
                web::scope("/api")
                    .route("/placeholder", web::get().to(HttpResponse::Ok))
                    .route("/placeholder", web::post().to(HttpResponse::Ok)),
            )
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
