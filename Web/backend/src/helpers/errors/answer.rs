use axum::response::{IntoResponse, Response};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AnswerControllerError {}

#[derive(Debug, Error)]
pub enum AnswerServiceError {
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}
impl IntoResponse for AnswerControllerError {
    fn into_response(self) -> Response {
        todo!()
    }
}
