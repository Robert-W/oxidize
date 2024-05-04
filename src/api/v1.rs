mod sample;

use crate::state::AppState;
use axum::Router;

pub(crate) fn routes() -> Router<AppState> {
    Router::new().nest("/sample", sample::routes())
}
