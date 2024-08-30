use std::process::ExitCode;

use tokio::net::TcpListener;

use crate::{api, state::AppState};

pub async fn run() -> ExitCode {
    // Construct our AppState
    let app_state = match AppState::new().await {
        Ok(state) => state,
        Err(err) => {
            tracing::error!({ exception.message = %err }, "Failure to construct AppState");
            return ExitCode::FAILURE;
        }
    };

    // Run migrations against our database
    if let Err(err) = sqlx::migrate!().run(&*app_state.pool).await {
        tracing::error!({ exception.message = %err }, "Failure to run migrations");
        return ExitCode::FAILURE;
    }

    // Create the server
    let router = api::router().with_state(app_state);
    let listener = match TcpListener::bind("0.0.0.0:3000").await {
        Ok(listener) => listener,
        Err(err) => {
            tracing::error!({ exception.message = %err }, "Failure to run migrations");
            return ExitCode::FAILURE;
        }
    };

    tracing::info!("Server listening on 0.0.0.0:3000");
    match axum::serve(listener, router).await {
        Ok(_) => ExitCode::SUCCESS,
        Err(err) => {
            tracing::error!({ exception.message = %err }, "Failure to start server");
            ExitCode::FAILURE
        }
    }
}
