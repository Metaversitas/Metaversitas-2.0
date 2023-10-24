pub mod answer;
pub mod auth;
pub mod classroom;
pub mod exam;
pub mod question;
pub mod student;
pub mod subject;
pub mod teacher;
pub mod user;

use axum::extract::rejection::JsonRejection;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::Json;
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error(transparent)]
    ValidationError(#[from] anyhow::Error),
    #[error(transparent)]
    JsonExtractorRejection(#[from] JsonRejection),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ApiError::ValidationError(err) => (StatusCode::UNPROCESSABLE_ENTITY, err.to_string()),
            ApiError::JsonExtractorRejection(json_rejection) => {
                (json_rejection.status(), json_rejection.to_string())
            }
        };

        let payload = json!({
            "message": message,
        });

        (status, Json(payload)).into_response()
    }
}
