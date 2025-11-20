use axum::http::StatusCode;

// Healthcheck function we'll use with load balanver configurations
pub async fn health() -> StatusCode {
    StatusCode::OK
}

// Readiness check to ensure the system is ready to take requests
#[tracing::instrument()]
pub async fn ready() -> StatusCode {
    StatusCode::OK
}
