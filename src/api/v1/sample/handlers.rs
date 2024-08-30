pub use crate::state::AppState;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use tracing::error;
use uuid::Uuid;

use super::{
    forms::{CreateSample, UpdateSample},
    model::Sample,
};

#[tracing::instrument(skip(state))]
pub(crate) async fn list(
    State(state): State<AppState>,
) -> Result<Json<Vec<Sample>>, StatusCode> {
    match Sample::list(&*state.pool).await {
        Ok(samples) => {
            Ok(Json(samples))
        }
        Err(err) => {
            error!({ exception.message = %err }, "Unexpected error on Sample::list");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[tracing::instrument(skip(state))]
pub(crate) async fn create(
    State(state): State<AppState>,
    Json(body): Json<CreateSample>,
) -> Result<Json<Sample>, StatusCode> {
    match Sample::create(&*state.pool, body).await {
        Ok(sample) => Ok(Json(sample)),
        Err(err) => {
            error!({ exception.message = %err }, "Unexpected error on Sample::create");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[tracing::instrument(skip(state))]
pub(crate) async fn read(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Sample>, StatusCode> {
    match Sample::read(&*state.pool, &id).await {
        Ok(sample) => Ok(Json(sample)),
        Err(err) => {
            error!({ exception.message = %err }, "Unexpected error on Sample::read");
            Err(StatusCode::NOT_FOUND)
        }
    }
}

#[tracing::instrument(skip(state))]
pub(crate) async fn update(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateSample>,
) -> Result<Json<Sample>, StatusCode> {
    // TODO
    // 200/204 for updating an existing resource
    // 201 if a new user is created
    match Sample::update(&*state.pool, &id, body).await {
        Ok(sample) => Ok(Json(sample)),
        Err(err) => {
            error!({ exception.message = %err }, "Unexpected error on Sample::update");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
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
