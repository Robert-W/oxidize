use sqlx::{Pool, Postgres};
use std::sync::Arc;

use crate::db::postgres;

#[derive(Clone)]
pub struct AppState {
    pub pool: Arc<Pool<Postgres>>,
}

impl AppState {
    pub async fn new() -> anyhow::Result<AppState> {
        let pool = postgres::create_pool().await?;

        Ok(AppState {
            pool: Arc::new(pool),
        })
    }
}
