use axum::response::{IntoResponse, Response};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ExamControllerError {}

impl IntoResponse for ExamControllerError {
    fn into_response(self) -> Response {
        todo!()
    }
}
