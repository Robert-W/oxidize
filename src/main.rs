mod api;
mod db;
mod state;
mod subscribers;

// #[cfg(test)]
mod tests;

use axum::Router;
use dotenvy::dotenv;
use state::AppState;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // Load our environment variables
    dotenv().ok();

    // Initialize tracing
    tracing::subscriber::set_global_default(subscribers::all()).unwrap();

    // Construct our AppState
    let app_state = AppState::new().await.expect("Unable to create app state");

    // Run migrations with out datbase
    sqlx::migrate!()
        .run(&app_state.pool)
        .await
        .expect("Migrations failed to run");

    // Setup Axum
    let router = Router::new()
        .nest("/api", api::routes())
        .with_state(app_state);

    let listener = TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Unable to bind to TcpListener");

    axum::serve(listener, router)
        .await
        .expect("Unable to start the server");
}
