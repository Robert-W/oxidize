use crate::state::AppState;
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
    responses::SampleResponse,
};

pub(crate) async fn list(State(state): State<AppState>) -> Result<Json<Vec<SampleResponse>>, StatusCode> {
    match Sample::list(&state.pool).await {
        Ok(samples) => {
            let response = samples
                .into_iter()
                .map(|s| SampleResponse::from(s))
                .collect();

            Ok(Json(response))
        }
        Err(err) => {
            error!("Encountered unexpected error on Sample::list. {:?}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub(crate) async fn create(
    State(state): State<AppState>,
    Json(body): Json<CreateSample>,
) -> Result<Json<SampleResponse>, StatusCode> {
    match Sample::create(&state.pool, body).await {
        Ok(sample) => Ok(Json(SampleResponse::from(sample))),
        Err(err) => {
            error!("Encountered unexpected error on Sample::create. {:?}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub(crate) async fn read(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<SampleResponse>, StatusCode> {
    match Sample::read(&state.pool, &id).await {
        Ok(sample) => Ok(Json(SampleResponse::from(sample))),
        Err(err) => {
            error!("Encountered unexpected error on Sample::read. {:?}", err);
            Err(StatusCode::NOT_FOUND)
        }
    }
}

pub(crate) async fn update(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateSample>,
) -> Result<Json<SampleResponse>, StatusCode> {
    // TODO
    // 200/204 for updating an existing resource
    // 201 if a new user is created
    match Sample::update(&state.pool, &id, body).await {
        Ok(sample) => Ok(Json(SampleResponse::from(sample))),
        Err(err) => {
            error!("Encountered unexpected error on Sample::update. {:?}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub(crate) async fn delete(Path(id): Path<Uuid>, State(state): State<AppState>) -> StatusCode {
    match Sample::delete(&state.pool, &id).await {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(_) => StatusCode::NO_CONTENT,
    }
}
