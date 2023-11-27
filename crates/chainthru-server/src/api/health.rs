use actix_web::{HttpResponse, Responder};

/// Used to check if the server is up and running
pub async fn health() -> impl Responder {
    HttpResponse::Ok()
}
