use thiserror::Error;

#[derive(Debug, Error)]
pub enum TeacherServiceError {
    #[error(transparent)]
    UuidParseFailed(#[from] uuid::Error),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}
