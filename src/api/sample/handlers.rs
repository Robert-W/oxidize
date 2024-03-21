use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sqlx::PgPool;
use tracing::error;
use uuid::Uuid;

use super::{
    forms::{CreateSample, UpdateSample},
    model::Sample,
    responses::SampleResponse,
};

pub async fn list(State(pool): State<PgPool>) -> Result<Json<Vec<SampleResponse>>, StatusCode> {
    match Sample::list(&pool).await {
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

pub async fn create(
    State(pool): State<PgPool>,
    Json(body): Json<CreateSample>,
) -> Result<Json<SampleResponse>, StatusCode> {
    match Sample::create(&pool, body).await {
        Ok(sample) => Ok(Json(SampleResponse::from(sample))),
        Err(err) => {
            error!("Encountered unexpected error on Sample::create. {:?}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn read(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<SampleResponse>, StatusCode> {
    match Sample::read(&pool, &id).await {
        Ok(sample) => Ok(Json(SampleResponse::from(sample))),
        Err(err) => {
            error!("Encountered unexpected error on Sample::read. {:?}", err);
            Err(StatusCode::NOT_FOUND)
        }
    }
}

pub async fn update(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateSample>,
) -> Result<Json<SampleResponse>, StatusCode> {
    // TODO
    // 200/204 for updating an existing resource
    // 201 if a new user is created
    match Sample::update(&pool, &id, body).await {
        Ok(sample) => Ok(Json(SampleResponse::from(sample))),
        Err(err) => {
            error!("Encountered unexpected error on Sample::update. {:?}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn delete(Path(id): Path<Uuid>, State(pool): State<PgPool>) -> StatusCode {
    match Sample::delete(&pool, &id).await {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(_) => StatusCode::NO_CONTENT,
    }
}
