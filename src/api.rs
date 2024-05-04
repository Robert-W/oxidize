mod v1;

use crate::observability::otel;
use crate::state::AppState;
use axum::{http::StatusCode, routing::get, Router};
use tower::ServiceBuilder;

pub(crate) fn router() -> Router<AppState> {
    Router::new()
        .merge(global_routes())
        .merge(versioned_routes())
        .layer(ServiceBuilder::new().layer(otel::layer()))
}

fn global_routes() -> Router<AppState> {
    Router::new().route("/health", get(health))
}

fn versioned_routes() -> Router<AppState> {
    Router::new().nest("/v/1", v1::routes())
}

/// Simple healthcheck function we'll use with load balanver configurations
async fn health() -> StatusCode {
    StatusCode::OK
}
