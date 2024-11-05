use crate::api::{errors::ApiError, response::ApiResponse};
pub use crate::state::AppState;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde_json::json;
use tracing::error;
use uuid::Uuid;

use super::{
    forms::{CreateSample, UpdateSample},
    model::Sample,
};

#[tracing::instrument(skip(state))]
pub(crate) async fn list(State(state): State<AppState>) -> ApiResponse {
    match Sample::list(&*state.pool).await {
        Ok(samples) => ApiResponse::ok(json!(samples)),
        Err(err) => {
            error!({ exception.message = %err }, "Unexpected error on Sample::list");
            ApiResponse::err(err.into())
        }
    }
}

#[tracing::instrument(skip(state))]
pub(crate) async fn create(
    State(state): State<AppState>,
    Json(body): Json<CreateSample>,
) -> ApiResponse {
    match Sample::create(&*state.pool, body).await {
        Ok(sample) => ApiResponse::ok(json!(sample)),
        Err(err) => {
            error!({ exception.message = %err }, "Unexpected error on Sample::create");
            ApiResponse::err(err.into())
        }
    }
}

#[tracing::instrument(skip(state))]
pub(crate) async fn read(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResponse {
    match Sample::read(&*state.pool, &id).await {
        Ok(sample) => ApiResponse::ok(json!(sample)),
        Err(err) => {
            error!({ exception.message = %err }, "Unexpected error on Sample::read");
            // If the error is rows not found, return a 404 instead
            match err {
                sqlx::Error::RowNotFound => ApiResponse::err(ApiError::NotFound.into()),
                other => ApiResponse::err(other.into())
            }

        }
    }
}

#[tracing::instrument(skip(state))]
pub(crate) async fn update(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateSample>,
) -> ApiResponse {
    // TODO
    // 200/204 for updating an existing resource
    // 201 if a new user is created
    match Sample::update(&*state.pool, &id, body).await {
        Ok(sample) => ApiResponse::ok(json!(sample)),
        Err(err) => {
            error!({ exception.message = %err }, "Unexpected error on Sample::update");
            ApiResponse::err(err.into())
        }
    }
}

#[tracing::instrument(skip(state))]
pub(crate) async fn delete(Path(id): Path<Uuid>, State(state): State<AppState>) -> StatusCode {
    match Sample::delete(&*state.pool, &id).await {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(err) => {
            error!({ exception.message = %err }, "Unexpected error on Sample::delete");
            StatusCode::NO_CONTENT
        }
    }
}
