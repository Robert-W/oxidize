mod api;
mod constants;
mod db;
mod monitoring;
mod server;
mod state;

use std::process::ExitCode;

use dotenvy::dotenv;

#[tokio::main]
async fn main() -> ExitCode {
    // Load our environment variables
    dotenv().ok();

    // Initialize tracing and logging
    monitoring::init().await;

    let exit_code = server::run().await;

    // Perform any cleanup
    monitoring::shutdown();

    exit_code
}
