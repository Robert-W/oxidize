mod forms;
mod handlers;
mod model;
mod responses;

use crate::state::AppState;
use axum::{
    routing::{delete, get, post, put},
    Router,
};

pub(crate) fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(handlers::list))
        .route("/", post(handlers::create))
        .route("/:id", get(handlers::read))
        .route("/:id", put(handlers::update))
        .route("/:id", delete(handlers::delete))
}
