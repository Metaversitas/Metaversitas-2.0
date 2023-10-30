use thiserror::Error;

#[derive(Error, Debug)]
pub enum StudentServiceError {
    #[error(transparent)]
    UuidParseError(#[from] uuid::Error),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}
