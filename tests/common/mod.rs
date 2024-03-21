use oxidize::db::postgres::{get_connection_string, create_pool};
use sqlx::PgPool;

pub async fn get_pool() -> PgPool {
    let db_uri = get_connection_string();

    create_pool(&db_uri)
        .await
        .expect("Unable to obtain test pool.")
}
