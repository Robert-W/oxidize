use actix_web::web;

mod forms;
mod handlers;
mod model;
mod responses;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/sample")
            .service(handlers::create_sample)
            .service(handlers::read_sample)
            .service(handlers::list_samples)
            .service(handlers::update_sample)
            .service(handlers::delete_sample),
    );
}
