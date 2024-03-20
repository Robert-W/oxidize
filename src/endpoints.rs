mod sample;

use axum::{http::StatusCode, routing::get, Router};
use sqlx::{Pool, Postgres};

async fn healthcheck() -> StatusCode {
    StatusCode::OK
}

pub fn configure(pool: Pool<Postgres>) -> Router {
    Router::new().nest(
        "/api",
        Router::new()
            .route("/healthcheck", get(healthcheck))
            .nest("/sample", sample::configure(pool)),
    )
}
