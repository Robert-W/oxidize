mod api;
mod db;
mod observability;
mod server;
mod state;

use std::process::ExitCode;

use dotenvy::dotenv;

#[tokio::main]
async fn main() -> ExitCode {
    // Load our environment variables
    dotenv().ok();

    // Initialize tracing and logging
    observability::init();

    let exit_code = server::run().await;

    // Perform any cleanup
    opentelemetry::global::shutdown_tracer_provider();

    exit_code
}
