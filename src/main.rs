mod db;
mod endpoints;
mod subscribers;

use axum::{serve, Router};
use db::postgres;
use dotenvy::dotenv;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
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

    // Setup Axum
    let app = Router::new()
        .nest("/api", endpoints::configure(pool));

    let listener = TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Unable to bind to TcpListener");

    serve(listener, app)
        .await
        .expect("Unable to start the server");
}
