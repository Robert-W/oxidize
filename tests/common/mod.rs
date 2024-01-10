use oxidize::db::postgres::{get_connection_string, create_pool};
use sqlx::PgPool;

// Create a macro to initialize the server. This helps us get around some
// complicated return types. TODO - Investigate this later.
//
// When using, import the following:
// - use actix_web::{App, web};
// - use oxidize::endpoints
//
#[macro_export]
macro_rules! create_server {
    ($pool: expr) => {
        App::new()
            .app_data(web::Data::new($pool.clone()))
            .configure(endpoints::configure)
    };
}

pub async fn get_pool() -> PgPool {
    let db_uri = get_connection_string();

    create_pool(&db_uri)
        .await
        .expect("Unable to obtain test pool.")
}
