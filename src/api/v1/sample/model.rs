use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use super::forms::{CreateSample, UpdateSample};

#[derive(Deserialize, Serialize, Debug, FromRow)]
pub struct Sample {
    pub id: Uuid,
    pub name: String,
    pub created: NaiveDateTime,
    pub last_updated: NaiveDateTime,
}

impl Sample {
    #[tracing::instrument(skip(pool))]
    pub async fn create(pool: &PgPool, form: CreateSample) -> Result<Sample, sqlx::Error> {
        let timestamp = Utc::now();

        let sample: Sample = sqlx::query_as(
            "
INSERT INTO samples (id, name, created, last_updated)
VALUES ($1, $2, $3, $4) RETURNING *
",
        )
        .bind(Uuid::new_v4())
        .bind(form.name)
        .bind(timestamp)
        .bind(timestamp)
        .fetch_one(pool)
        .await?;

        Ok(sample)
    }

    #[tracing::instrument(skip(pool))]
    pub async fn read(pool: &PgPool, id: &Uuid) -> Result<Sample, sqlx::Error> {
        let sample: Sample = sqlx::query_as("SELECT * FROM samples WHERE id = $1")
            .bind(id)
            .fetch_one(pool)
            .await?;

        Ok(sample)
    }

    #[tracing::instrument(skip(pool))]
    pub async fn list(pool: &PgPool) -> Result<Vec<Sample>, sqlx::Error> {
        let samples: Vec<Sample> = sqlx::query_as("SELECT * FROM samples")
            .fetch_all(pool)
            .await?;

        Ok(samples)
    }

    #[tracing::instrument(skip(pool))]
    pub async fn update(
        pool: &PgPool,
        id: &Uuid,
        form: UpdateSample,
    ) -> Result<Sample, sqlx::Error> {
        let sample: Sample = sqlx::query_as(
            "UPDATE samples SET name = $1, last_updated = $2 WHERE id = $3 RETURNING *",
        )
        .bind(form.name)
        .bind(Utc::now())
        .bind(id)
        .fetch_one(pool)
        .await?;

        Ok(sample)
    }

    #[tracing::instrument(skip(pool))]
    pub async fn delete(pool: &PgPool, id: &Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE from samples WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;

        Ok(())
    }
}
