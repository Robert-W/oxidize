mod db;
mod endpoints;
mod subscribers;

use actix_web::{main, web, App, HttpServer};
use db::postgres;
use dotenvy::dotenv;

#[main]
async fn main() -> std::io::Result<()> {
    // Load our environment variables
    dotenv().ok();

    // Initialize tracing
    tracing::subscriber::set_global_default(subscribers::all()).unwrap();

    // Connect to our database
    let pg_conn_string = postgres::get_connection_string();
    let pool = postgres::create_pool(&pg_conn_string)
        .await
        .expect("Unable to connect to postgres");

    // Run migrations with out datbase
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Migrations failed to run");

    // Create our HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(endpoints::configure)
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}
