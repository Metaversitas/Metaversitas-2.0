use crate::helpers::errors::auth::AuthError;
use crate::helpers::errors::subject::SubjectServiceError;
use crate::helpers::errors::teacher::TeacherServiceError;
use anyhow::anyhow;
use axum::extract::rejection::JsonRejection;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::Json;
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClassroomServiceError {
    #[error("Unable to create a classroom with student access.")]
    UnauthorizedStudent,
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

#[derive(Error, Debug)]
pub enum ClassroomControllerError {
    #[error("Unable to create classroom")]
    UnableCreateClass,
    #[error("Unauthorized Access")]
    Unauthorized,
    #[error(transparent)]
    Other(#[from] anyhow::Error),
    #[error(transparent)]
    JsonRejection(#[from] JsonRejection),
    #[error("Unknown error happened")]
    Unknown,
}

impl IntoResponse for ClassroomControllerError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ClassroomControllerError::Unauthorized => {
                (StatusCode::UNAUTHORIZED, "Unauthorized access!".to_string())
            }
            ClassroomControllerError::Other(err) => {
                tracing::error!("{}", err.to_string());
                (StatusCode::INTERNAL_SERVER_ERROR, "".to_string())
            }
            ClassroomControllerError::Unknown => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error.".to_string(),
            ),
            ClassroomControllerError::UnableCreateClass => (
                StatusCode::FORBIDDEN,
                "Forbidden to create a classroom".to_string(),
            ),
            ClassroomControllerError::JsonRejection(err) => {
                (StatusCode::UNPROCESSABLE_ENTITY, err.to_string())
            }
        };
        let payload = json!({"message": message});
        (status, Json(payload)).into_response()
    }
}

impl From<AuthError> for ClassroomControllerError {
    fn from(err: AuthError) -> Self {
        match err {
            AuthError::Unauthorized => ClassroomControllerError::Unauthorized,
            AuthError::Other(err) => ClassroomControllerError::Other(err),
            _ => ClassroomControllerError::Unknown,
        }
    }
}

impl From<ClassroomServiceError> for ClassroomControllerError {
    fn from(err: ClassroomServiceError) -> Self {
        match err {
            ClassroomServiceError::UnauthorizedStudent => ClassroomControllerError::Unauthorized,
            ClassroomServiceError::UnexpectedError(err) => {
                ClassroomControllerError::Other(anyhow!("{}", err.to_string()))
            }
        }
    }
}

impl From<TeacherServiceError> for ClassroomControllerError {
    fn from(err: TeacherServiceError) -> Self {
        match err {
            TeacherServiceError::UuidParseFailed(err) => {
                ClassroomControllerError::Other(anyhow!("{}", err.to_string()))
            }
            TeacherServiceError::UnexpectedError(err) => {
                ClassroomControllerError::Other(anyhow!("{}", err.to_string()))
            }
        }
    }
}

impl From<SubjectServiceError> for ClassroomServiceError {
    fn from(err: SubjectServiceError) -> Self {
        match err {
            SubjectServiceError::NotFound => {
                ClassroomServiceError::UnexpectedError(anyhow!("Not found any subject"))
            }
            SubjectServiceError::UnexpectedError(err) => {
                ClassroomServiceError::UnexpectedError(anyhow!("{}", err.to_string()))
            }
        }
    }
}
