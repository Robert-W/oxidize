mod sample;

use axum::{http::StatusCode, routing::get, Router};

use crate::state::AppState;

async fn health() -> StatusCode {
    StatusCode::OK
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/health", get(health))
        .nest("/sample", sample::routes())
}
