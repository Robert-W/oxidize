pub mod common;

use axum::{
    body::Body,
    extract::Request,
    http::StatusCode,
    Router,
};
use common::get_pool;
use oxidize::api;
use tower::ServiceExt;

#[cfg(test)]
mod healthcheck_tests {
    use super::*;

    #[tokio::test]
    pub async fn healthcheck() {
        let pool = get_pool().await;
        let app = Router::new().nest("/api", api::routes()).with_state(pool);

        let request = Request::builder()
            .uri("/api/healthcheck")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
