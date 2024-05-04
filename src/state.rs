use sqlx::{Pool, Postgres};

use crate::db::postgres;

#[derive(Clone)]
pub(crate) struct AppState {
    pub pool: Pool<Postgres>,
}

impl AppState {
    pub async fn new() -> anyhow::Result<AppState> {
        let pool = postgres::create_pool().await?;

        Ok(AppState { pool })
    }
}
