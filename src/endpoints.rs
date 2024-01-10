mod sample;

use actix_web::{get, web, HttpResponse, Responder};

#[get("/healthcheck")]
async fn healthcheck() -> impl Responder {
    HttpResponse::Ok()
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(healthcheck)
            .configure(sample::configure),
    );
}
