use crate::helpers::errors::auth::AuthError;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SubjectServiceError {
    #[error("Not found a subject")]
    NotFound,
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

#[derive(Debug, Error)]
pub enum SubjectControllerError {
    #[error("Unauthorized Access")]
    Unauthorized,
    #[error(transparent)]
    Other(#[from] anyhow::Error),
    #[error("Unknown Error")]
    Unknown,
}

impl IntoResponse for SubjectControllerError {
    fn into_response(self) -> Response {
        match self {
            SubjectControllerError::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                Json(json!({"message": "Unauthorized access!"})),
            )
                .into_response(),
            SubjectControllerError::Unknown => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Internal server error."})),
            )
                .into_response(),
            SubjectControllerError::Other(err) => {
                let err_msg = format!("Internal server error. With reason: {}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"message": err_msg})),
                )
                    .into_response()
            }
        }
    }
}

impl From<AuthError> for SubjectControllerError {
    fn from(err: AuthError) -> Self {
        match err {
            AuthError::Unauthorized => SubjectControllerError::Unauthorized,
            AuthError::Other(err) => SubjectControllerError::Other(err),
            _ => SubjectControllerError::Unknown,
        }
    }
}
