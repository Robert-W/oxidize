use axum::http::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Resource not found")]
    NotFound,
    #[error("{source}")]
    Database { source: sqlx::Error },
}

impl ApiError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::Database { .. } => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<sqlx::Error> for ApiError {
    fn from(value: sqlx::Error) -> Self {
        Self::Database { source: value }
    }
}
