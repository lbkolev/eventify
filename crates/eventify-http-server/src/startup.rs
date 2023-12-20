use std::net::TcpListener;

use actix_web::{dev::Server, web, App, HttpServer};
use sqlx::{postgres::PgPoolOptions, PgPool};
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;

use eventify_primitives::config::{DatabaseConfig, ServerConfig};

use crate::{
    api::{self, block, log, transaction},
    Result,
};

#[allow(missing_debug_implementations)]
pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(settings: ServerConfig) -> Result<Self> {
        let connection_pool = get_connection_pool(&settings.database);
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

pub fn get_connection_pool(configuration: &DatabaseConfig) -> PgPool {
    PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        // The connection needs to be established lazily, since the database
        // might not be available when the application starts.
        .connect_lazy_with(configuration.with_db())
}

#[derive(OpenApi)]
#[openapi(paths(
    // block
    block::get_blocks_count,

    // tx
    transaction::get_transactions_count,

    // log
    log::get_logs_count
))]
struct ApiDoc;

pub fn start(
    listener: TcpListener,
    worker_threads: usize,
    db_pool: PgPool,
) -> std::result::Result<Server, std::io::Error> {
    let db_pool = web::Data::new(db_pool);

    let openapi = ApiDoc::openapi();

    let server = HttpServer::new(move || {
        App::new()
            // swagger-related
            .service(Redoc::with_url("/redoc", openapi.clone()))
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
            .service(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
            // api
            .route("/health", web::get().to(api::health))
            .service(
                web::scope("/api").service(
                    web::scope("/v1")
                        .service(web::scope("/blocks").service(block::get_blocks_count))
                        .service(
                            web::scope("/transactions")
                                .service(transaction::get_transactions_count),
                        )
                        .service(web::scope("/logs").service(api::log::get_logs_count)),
                ),
            )
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .workers(worker_threads)
    .run();

    Ok(server)
}
