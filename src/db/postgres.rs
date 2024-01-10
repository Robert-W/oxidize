use sqlx::postgres::{PgPool, PgPoolOptions};
use std::env;

pub fn get_connection_string() -> String {
    env::var("DATABASE_URL").expect("Missing DATABASE_URL environment variable")
}

pub async fn create_pool(conn_string: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(8)
        .connect(conn_string)
        .await
}
