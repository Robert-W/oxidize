use axum::{
    routing::{delete, get, post, put},
    Router,
};

use crate::state::AppState;

mod forms;
mod handlers;
mod model;
mod responses;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(handlers::list))
        .route("/", post(handlers::create))
        .route("/:id", get(handlers::read))
        .route("/:id", put(handlers::update))
        .route("/:id", delete(handlers::delete))
}
