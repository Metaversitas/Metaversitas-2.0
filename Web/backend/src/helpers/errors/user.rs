use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserServiceError {
    #[error("Can not connect into Database.")]
    DatabaseConnectionError,
    #[error("User does not exist.")]
    UserDoesNotExist,
    #[error("Password does not match.")]
    PasswordNotMatch,
    #[error("Unable to create a session.")]
    UnableCreateSession,
    #[error("User already registered.")]
    UserAlreadyExists,
    #[error("Failed to hash password.")]
    UnableHashPassword,
    #[error("Can not connect into Redis.")]
    RedisConnectionError,
    #[error("Unauthorized access.")]
    UnauthorizedAccess,
    #[error("Unable parse incoming data")]
    UnableToParse,
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}
