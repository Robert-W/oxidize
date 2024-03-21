mod sample;

use axum::{http::StatusCode, routing::get, Router};
use sqlx::{Pool, Postgres};

async fn healthcheck() -> StatusCode {
    StatusCode::OK
}

pub fn routes() -> Router<Pool<Postgres>> {
    Router::new()
        .route("/healthcheck", get(healthcheck))
        .nest("/sample", sample::routes())
}
