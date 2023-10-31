use thiserror::Error;

#[derive(Debug, Error)]
pub enum SubjectServiceError {
    #[error("Not found a subject")]
    NotFound,
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}
