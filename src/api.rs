mod errors;
mod response;
mod system;
mod v1;

use std::time::Duration;

use crate::{monitoring::trace, state::AppState};
use axum::{routing::get, Router};
use tower::ServiceBuilder;
use tower_http::{compression::CompressionLayer, timeout::TimeoutLayer, CompressionLevel};

pub(crate) fn router() -> Router<AppState> {
    Router::new()
        .merge(versioned_routes())
        .nest("/system", system_routes())
        .layer(ServiceBuilder::new().layer(trace::new_trace_layer()))
        .layer(
            CompressionLayer::new()
                .br(true)
                .quality(CompressionLevel::Fastest),
        )
        // Graceful shutdown will tell axum to wait for requests to complete
        // We don't want that to run on indefinitely
        .layer(TimeoutLayer::new(Duration::from_secs(10)))
}

fn system_routes() -> Router<AppState> {
    Router::new()
        .route("/health", get(system::health))
        .route("/ready", get(system::ready))
}

fn versioned_routes() -> Router<AppState> {
    Router::new().nest("/v/1", v1::routes())
}
