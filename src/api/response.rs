use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use serde_json::Value;

use super::errors::ApiError;

/// This defines the response format that all of our API endpoints will use
/// It enforces outcomes to conform to one of the following formats:
///
/// Success: { status: 'Ok', result: T }
/// Error: { status: 'Err', error: ServiceError }
#[derive(Debug, Serialize)]
#[serde(tag = "status")]
pub enum ApiResponse {
    Ok { result: Value },
    Err { error: ServiceError },
}

impl ApiResponse {
    /// Use `ApiResponse::ok` to create a successful ApiResponse
    pub fn ok(result: Value) -> Self {
        ApiResponse::Ok { result }
    }

    /// Use `ApiResponse::err` to create a failure ApiResponse
    pub fn err(error: ServiceError) -> Self {
        ApiResponse::Err { error }
    }
}

impl IntoResponse for ApiResponse {
    fn into_response(self) -> Response {
        let status = match self {
            ApiResponse::Ok { .. } => StatusCode::OK,
            ApiResponse::Err {
                error: ServiceError { status_code, .. },
            } => status_code,
        };

        (status, axum::Json(self)).into_response()
    }
}

/// Stucture of the response created when using `ApiResponse::err`
/// status_code is just used to determine the approriate status for your error
/// see ./errors.rs, `enum ApiError` to add more variants.
#[derive(Debug, Serialize)]
pub struct ServiceError {
    #[serde(skip)]
    pub status_code: StatusCode,
    pub message: String,
}

impl From<sqlx::Error> for ServiceError {
    fn from(value: sqlx::Error) -> Self {
        ApiError::from(value).into()
    }
}

impl From<ApiError> for ServiceError {
    fn from(value: ApiError) -> Self {
        ServiceError {
            status_code: value.status_code(),
            message: value.to_string(),
        }
    }
}

impl IntoResponse for ServiceError {
    fn into_response(self) -> Response {
        let result = ApiResponse::err(self);

        result.into_response()
    }
}
