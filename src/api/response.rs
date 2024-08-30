/// This defines the response format that all of our API endpoints will use
/// It enforces outcomes to conform to one of the following formats:
///
/// Success: { status: 'Ok', result: T }
/// Error: { status: 'Err', error: ServiceError }
///
/// see https://github.com/bubblegroup/heimdall/pull/81/files#diff-3ffcb125efa2c5870ce050e19274feb0935ed9056e89546d2dcb1848773c1bc4
use axum::{
    http::StatusCode, response::{IntoResponse, Response}
};
use serde::Serialize;
use serde_json::Value;

use super::errors::ApiError;

#[derive(Debug, Serialize)]
#[serde(tag = "status")]
pub enum ApiResponse {
    Ok { result: Value },
    Err { error: ServiceError },
}

impl From<Result<Value, ServiceError>> for ApiResponse {
    fn from(value: Result<Value, ServiceError>) -> Self {
        match value {
            Ok(result) => ApiResponse::Ok { result },
            Err(error) => ApiResponse::Err { error },
        }
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
            message: value.to_string()
        }
    }
}

impl IntoResponse for ServiceError {
    fn into_response(self) -> Response {
        let result = ApiResponse::from(Err(self));

        result.into_response()
    }
}
