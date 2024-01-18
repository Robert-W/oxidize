use actix_web::{
    delete, get, post, put,
    web::{Data, Form, Path, Json},
    Error, HttpResponse, Responder,
};
use tracing::error;
use sqlx::PgPool;
use uuid::Uuid;

use super::{
    forms::{CreateSample, UpdateSample},
    model::Sample,
    responses::SampleResponse,
};

#[post("/")]
pub async fn create_sample(
    form: Form<CreateSample>,
    pool: Data<PgPool>,
) -> Result<SampleResponse, Error> {
    match Sample::create(&pool, form.into_inner()).await {
        Ok(sample) => Ok(SampleResponse::from(sample)),
        Err(err) => {
            error!("Encountered unexpected error on Sample::create. {:?}", err);
            Err(actix_web::error::ErrorInternalServerError(err))
        }
    }
}

#[get("/{id}")]
async fn read_sample(id: Path<Uuid>, pool: Data<PgPool>) -> Result<SampleResponse, Error> {
    match Sample::read(&pool, &id).await {
        Ok(sample) => Ok(SampleResponse::from(sample)),
        Err(err) => {
            error!("Encountered unexpected error on Sample::read. {:?}", err);
            Err(actix_web::error::ErrorNotFound(err))
        }
    }
}

#[get("/")]
async fn list_samples(pool: Data<PgPool>) -> Result<Json<Vec<SampleResponse>>, Error> {
    match Sample::list(&pool).await {
        Ok(samples) => {
            let response = samples.into_iter().map(|s| SampleResponse::from(s)).collect();

            Ok(Json(response))
        },
        Err(err) => {
            error!("Encountered unexpected error on Sample::list. {:?}", err);
            Err(actix_web::error::ErrorInternalServerError(err))
        }
    }
}

#[put("/{id}")]
async fn update_sample(form: Form<UpdateSample>, id: Path<Uuid>, pool: Data<PgPool>) -> Result<SampleResponse, Error> {
    // TODO
    // 200/204 for updating an existing resource
    // 201 if a new user is created
    match Sample::update(&pool, &id, form.into_inner()).await {
        Ok(sample) => Ok(SampleResponse::from(sample)),
        Err(err) => {
            error!("Encountered unexpected error on Sample::update. {:?}", err);
            Err(actix_web::error::ErrorInternalServerError(err))
        }
    }
}

#[delete("/{id}")]
async fn delete_sample(id: Path<Uuid>, pool: Data<PgPool>) -> impl Responder {
    match Sample::delete(&pool, &id).await {
        Ok(_) => HttpResponse::NoContent(),
        Err(_) => HttpResponse::NoContent(),
    }
}
