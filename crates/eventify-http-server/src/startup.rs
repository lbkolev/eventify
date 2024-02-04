use std::net::TcpListener;

use actix_web::{dev::Server, web, App, HttpServer};
use sqlx::{PgPool, Pool, Postgres};
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;

use eventify_configs::configs::ApplicationConfig;

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
    pub async fn build(config: ApplicationConfig, connection_pool: Pool<Postgres>) -> Result<Self> {
        let listener = TcpListener::bind(format!("{}:{}", config.server.host, config.server.port))?;
        let port = listener.local_addr().unwrap().port();
        let server = start(listener, connection_pool)?;

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> std::result::Result<(), std::io::Error> {
        self.server.await
    }
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
    .disable_signals()
    .run();

    Ok(server)
}
