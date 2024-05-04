use sqlx::{Pool, Postgres};

use crate::db::postgres;

#[derive(Clone)]
pub(crate) struct AppState {
    pub pool: Pool<Postgres>,
}

impl AppState {
    pub async fn new() -> anyhow::Result<AppState> {
        let connection_string = postgres::get_connection_string();
        let pool = postgres::create_pool(&connection_string).await?;

        Ok(AppState { pool })
    }
}
