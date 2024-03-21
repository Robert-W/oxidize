pub mod common;

use axum::{body::Body, extract::Request, http::{self, StatusCode}, Router};
use common::get_pool;
use oxidize::api;
use serde_json::{json, Value};
use sqlx::{Pool, Postgres};
use tower::ServiceExt;

fn build_router(pool: Pool<Postgres>) -> Router {
    Router::new().nest("/api", api::routes()).with_state(pool)
}

#[cfg(test)]
mod sample_tests {
    use super::*;

    #[tokio::test]
    pub async fn healthchecks() {
        let pool = get_pool().await;
        let app = build_router(pool);

        let request = Request::builder()
            .uri("/api/healthcheck")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    pub async fn create() {
        let pool = get_pool().await;
        let app = build_router(pool);

        // let request = Request::builder()
        //     .method(http::Method::POST)
        //     .uri("/api/sample")
        //     .body(
        //         Body::from(
        //             serde_json::from_value(json!({ "name": "scrappy" })).unwrap()
        //         )
        //     )
        //     .unwrap();

    }
}
