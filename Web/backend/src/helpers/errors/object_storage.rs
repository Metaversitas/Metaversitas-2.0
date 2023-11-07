use thiserror::Error;

#[derive(Error, Debug)]
pub enum ObjectStorageError {
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}
