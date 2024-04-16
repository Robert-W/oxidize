use axum::{body::Body, extract::Request, http::StatusCode, Router};
use tower::ServiceExt;

use crate::api;
use crate::state::AppState;

#[cfg(test)]
mod healthcheck_tests {
    use super::*;

    #[tokio::test]
    pub async fn healthcheck() {
        let state = AppState::new().await.unwrap();
        let app = Router::new().nest("/api", api::routes()).with_state(state);

        let request = Request::builder()
            .uri("/api/health")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
