use actix_web::get;
use actix_web::{HttpResponse, Responder};

#[get("/health-check")]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
