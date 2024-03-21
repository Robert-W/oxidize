use sqlx::postgres::{PgPool, PgPoolOptions};
use std::{env, time::Duration};

pub fn get_connection_string() -> String {
    env::var("DATABASE_URL").expect("Missing DATABASE_URL environment variable")
}

pub async fn create_pool(conn_string: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .acquire_timeout(Duration::from_secs(30))
        .max_connections(32)
        .connect(conn_string)
        .await
}
