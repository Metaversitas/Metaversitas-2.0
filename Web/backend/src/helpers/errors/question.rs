use crate::helpers::errors::auth::AuthError;
use axum::extract::rejection::JsonRejection;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum QuestionControllerError {
    #[error(transparent)]
    JsonRejection(#[from] JsonRejection),
    #[error(transparent)]
    ErrorWithMessage(#[from] anyhow::Error),
    #[error("Unauthorized access.")]
    UnauthorizedAccess,
    #[error("Unknown error")]
    Unknown,
}

#[derive(Debug, Error)]
pub enum QuestionServiceError {
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl IntoResponse for QuestionControllerError {
    fn into_response(self) -> Response {
        match self {
            QuestionControllerError::ErrorWithMessage(err) => {
                let error_message = err.to_string();
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"message": error_message})),
                )
                    .into_response()
            }
            QuestionControllerError::UnauthorizedAccess => (
                StatusCode::UNAUTHORIZED,
                Json(json!({"message": "Unauthorized Access."})),
            )
                .into_response(),
            QuestionControllerError::Unknown => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Internal server error."})),
            )
                .into_response(),
            QuestionControllerError::JsonRejection(err) => {
                let error_message = err.to_string();
                (
                    StatusCode::UNPROCESSABLE_ENTITY,
                    Json(json!({"message": error_message})),
                )
                    .into_response()
            }
        }
    }
}

impl From<AuthError> for QuestionControllerError {
    fn from(value: AuthError) -> Self {
        match value {
            AuthError::Unauthorized => QuestionControllerError::UnauthorizedAccess,
            _ => QuestionControllerError::Unknown,
        }
    }
}
