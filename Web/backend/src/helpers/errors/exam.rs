use crate::helpers::errors::auth::AuthError;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ExamControllerError {
    #[error("Unauthorized")]
    Unauthorized,
    #[error(transparent)]
    ErrorWithMessage(#[from] anyhow::Error),
    #[error("Unknown")]
    Unknown,
}

#[derive(Debug, Error)]
pub enum ExamServiceError {
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl IntoResponse for ExamControllerError {
    fn into_response(self) -> Response {
        match self {
            ExamControllerError::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                Json(json!({"message": "Unauthorized access."})),
            )
                .into_response(),
            ExamControllerError::ErrorWithMessage(err) => {
                let err_message = err.to_string();
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"message": err_message})),
                )
                    .into_response()
            }
            ExamControllerError::Unknown => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Internal server error."})),
            )
                .into_response(),
        }
    }
}

impl From<AuthError> for ExamControllerError {
    fn from(err: AuthError) -> Self {
        match err {
            AuthError::Unauthorized => ExamControllerError::Unauthorized,
            AuthError::Unknown => ExamControllerError::Unknown,
            AuthError::Other(err) => ExamControllerError::ErrorWithMessage(err),
            _ => ExamControllerError::Unknown,
        }
    }
}
