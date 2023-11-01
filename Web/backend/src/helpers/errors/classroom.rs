use crate::helpers::errors::auth::AuthError;
use crate::helpers::errors::subject::SubjectServiceError;
use crate::helpers::errors::teacher::TeacherServiceError;
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
    #[error("Student has a schedule that conflict.")]
    StudentHasScheduleConflict,
    #[error("Classroom is not exists")]
    ClassroomIsNotExists,
    #[error("Student isn't exists")]
    StudentIsNotExists,
    #[error("Classroom already full")]
    ClassroomFull,
    #[error("Student already enrolled")]
    StudentAlreadyEnrolled,
    #[error("Lecturer can't enroll classes")]
    LecturerNotAbleToEnroll,
    #[error("Unable to create classroom")]
    UnableCreateClass,
    #[error("Unauthorized Access")]
    Unauthorized,
    #[error(transparent)]
    Other(#[from] anyhow::Error),
    #[error(transparent)]
    JsonRejection(#[from] JsonRejection),
    #[error(transparent)]
    ClassroomServiceError(#[from] ClassroomServiceError),
    #[error(transparent)]
    SubjectServiceError(#[from] SubjectServiceError),
    #[error(transparent)]
    TeacherServiceError(#[from] TeacherServiceError),
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
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Internal server error, reason: {}", err.to_string()),
                )
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
            ClassroomControllerError::LecturerNotAbleToEnroll => (
                StatusCode::FORBIDDEN,
                "Lecturer should not be able to enroll a class without being pointed.".to_string(),
            ),
            ClassroomControllerError::StudentAlreadyEnrolled => (
                StatusCode::FORBIDDEN,
                "Student already enrolled!".to_string(),
            ),
            ClassroomControllerError::ClassroomFull => {
                (StatusCode::FORBIDDEN, "Classroom already full!".to_string())
            }
            ClassroomControllerError::StudentIsNotExists => {
                (StatusCode::FORBIDDEN, "Student is not exists.".to_string())
            }
            ClassroomControllerError::ClassroomIsNotExists => (
                StatusCode::FORBIDDEN,
                "Classroom is not exists.".to_string(),
            ),
            ClassroomControllerError::StudentHasScheduleConflict => (
                StatusCode::FORBIDDEN,
                "Student already have a schedule that conflict with current requested class."
                    .to_string(),
            ),
            ClassroomControllerError::ClassroomServiceError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error.".to_string(),
            ),
            ClassroomControllerError::SubjectServiceError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error.".to_string(),
            ),
            ClassroomControllerError::TeacherServiceError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error.".to_string(),
            ),
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
