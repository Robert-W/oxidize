use sqlx::postgres::{PgPool, PgPoolOptions};
use std::{env, time::Duration};

pub(crate) async fn create_pool() -> Result<PgPool, sqlx::Error> {
    let connection_string = get_connection_string();

    PgPoolOptions::new()
        .acquire_timeout(Duration::from_secs(30))
        .max_connections(32)
        .connect(&connection_string)
        .await
}

fn get_connection_string() -> String {
    let user = env::var("DB_USER").unwrap_or(String::from("username"));
    let pass = env::var("DB_PASS").unwrap_or(String::from("password"));
    let host = env::var("DB_HOST").unwrap_or(String::from("localhost"));
    let port = env::var("DB_PORT").unwrap_or(String::from("5432"));
    let name = env::var("DB_NAME").unwrap_or(String::from("urban_potato"));

    format!("postgresql://{user}:{pass}@{host}:{port}/{name}")
}
