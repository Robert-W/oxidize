pub mod common;

use axum::{
    body::Body,
    extract::Request,
    http::{self, StatusCode},
    Router,
};
use common::get_pool;
use http_body_util::BodyExt;
use oxidize::api;
use serde_json::{json, Value};
use sqlx::{Pool, Postgres};
use tower::ServiceExt;

// NOTE: Hardcoded ids and/or names in this file come
// from ./fixtures/20240109021054_insert_samples.sql

fn build_router(pool: Pool<Postgres>) -> Router {
    Router::new().nest("/api", api::routes()).with_state(pool)
}

#[cfg(test)]
mod sample_tests {
    use super::*;

    #[tokio::test]
    pub async fn create() {
        let pool = get_pool().await;
        let app = build_router(pool);
        let input = json!({ "name": "scrappy" });
        let json_string = serde_json::to_string(&input).unwrap();

        let request = Request::builder()
            .method(http::Method::POST)
            .header("Content-Type", "application/json")
            .uri("/api/sample")
            .body(Body::from(json_string))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let output: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(output["name"], "scrappy");
    }

    #[tokio::test]
    pub async fn read() {
        let pool = get_pool().await;
        let app = build_router(pool);
        let id = "0ef309be-dd16-447d-84c1-ec47cd8c1a8c";

        let request = Request::builder()
            .uri(format!("/api/sample/{}", id))
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let output: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(output["name"], "shaggy");
    }

    #[tokio::test]
    pub async fn list() {
        let pool = get_pool().await;
        let app = build_router(pool);

        let request = Request::builder()
            .uri("/api/sample")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let output: Value = serde_json::from_slice(&body).unwrap();
        let results = output.as_array().unwrap();
        // 5 resources are inserted via fixtures
        assert!(results.len() >= 5);
    }

    #[tokio::test]
    pub async fn update() {
        let pool = get_pool().await;
        let app = build_router(pool);
        let id = "0ef309be-dd16-447d-84c1-ec47cd8c1a8c";
        let input = json!({ "name": "steve" });
        let json_string = serde_json::to_string(&input).unwrap();

        let request = Request::builder()
            .method(http::Method::PUT)
            .header("Content-Type", "application/json")
            .uri(format!("/api/sample/{}", id))
            .body(Body::from(json_string))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let output: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(output["name"], "steve");
    }

    #[tokio::test]
    pub async fn delete() {
        let pool = get_pool().await;
        let app = build_router(pool.clone());
        let id = "93ee5b24-8c2d-42e7-9ed8-6f4eca7cad9a";

        let request = Request::builder()
            .method(http::Method::DELETE)
            .uri(format!("/api/sample/{}", id))
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::NO_CONTENT);

        // Fred should now be long gone
        // run a query to see if he actually is gone
        let request = Request::builder()
            .uri(format!("/api/sample/{}", id))
            .body(Body::empty())
            .unwrap();

        let app = build_router(pool.clone());
        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
