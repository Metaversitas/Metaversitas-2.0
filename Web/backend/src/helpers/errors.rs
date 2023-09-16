use axum::extract::rejection::JsonRejection;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::Json;
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("invalid json data")]
    ValidationError,
    #[error(transparent)]
    JsonExtractorRejection(#[from] JsonRejection),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ApiError::ValidationError => (StatusCode::UNPROCESSABLE_ENTITY, "Invalid JSON Body"),
            ApiError::JsonExtractorRejection(json_rejection) => {
                (json_rejection.status(), "Invalid JSON Body")
            }
        };

        let payload = json!({
            "message": message,
        });

        (status, Json(payload)).into_response()
    }
}

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Invalid password or username")]
    InvalidUsernameOrPassword,
    #[error("Unauthorized Access")]
    Unauthorized,
    #[error("Unknown token format")]
    UnknownTokenFormat,
    #[error("User already registered")]
    UserRegistered,
    #[error("Unable to create session")]
    UnableCreateSession,
    #[error("Internal server error")]
    DatabaseError,
    #[error("Internal server error")]
    Unknown,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AuthError::InvalidUsernameOrPassword => {
                (StatusCode::UNAUTHORIZED, "Invalid username or password")
            }
            AuthError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized access"),
            AuthError::UnknownTokenFormat => {
                (StatusCode::UNPROCESSABLE_ENTITY, "Unknown format of token")
            }
            AuthError::UserRegistered => (StatusCode::CONFLICT, "User already registered"),
            AuthError::UnableCreateSession => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
            }
            AuthError::DatabaseError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
            }
            AuthError::Unknown => (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error"),
        };
        let payload = json!({ "message": message });

        (status, Json(payload)).into_response()
    }
}
