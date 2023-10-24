use axum::response::{IntoResponse, Response};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AnswerControllerError {}

impl IntoResponse for AnswerControllerError {
    fn into_response(self) -> Response {
        todo!()
    }
}
