use axum::response::{IntoResponse, Response};
use thiserror::Error;
use crate::helpers::errors::auth::AuthError;

#[derive(Debug, Clone, Error)]
pub enum QuestionControllerError {
    #[error("Unauthorized access.")]
    UnauthorizedAccess,
    #[error("Unknown error")]
    Unknown,
}

#[derive(Debug, Clone, Error)]
pub enum QuestionServiceError {}

impl IntoResponse for QuestionControllerError {
    fn into_response(self) -> Response {
        todo!()
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
