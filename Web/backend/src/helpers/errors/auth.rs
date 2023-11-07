use crate::controllers::auth::AuthFormatType;
use crate::helpers::errors::user::UserServiceError;
use crate::service::game::GameServiceError;
use anyhow::anyhow;
use axum::extract::rejection::{JsonRejection, QueryRejection};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::Json;
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Outdated game version.")]
    OutdatedGameVersion,
    #[error("Invalid game version.")]
    InvalidGameVersion,
    #[error("User does not exist.")]
    UserNotExist,
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
    RedisError,
    #[error("Internal server error")]
    Unknown,
    #[error(transparent)]
    JsonExtractorRejection(#[from] JsonRejection),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        match self {
            AuthError::InvalidUsernameOrPassword => (
                StatusCode::UNAUTHORIZED,
                Json(json!({"message": "Invalid username or password"})),
            )
                .into_response(),
            AuthError::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                Json(json!({"message": "Unauthorized access"})),
            )
                .into_response(),

            AuthError::UnknownTokenFormat => (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(json!({"message": "Unknown format of token"})),
            )
                .into_response(),
            AuthError::UserRegistered => (
                StatusCode::CONFLICT,
                Json(json!({"message": "User already registered"})),
            )
                .into_response(),
            AuthError::UnableCreateSession => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Internal Server Error"})),
            )
                .into_response(),
            AuthError::DatabaseError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Internal Server Error"})),
            )
                .into_response(),
            AuthError::Unknown => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Internal Server Error"})),
            )
                .into_response(),
            AuthError::Other(error) => {
                tracing::error!("Got an error: {}", error.to_string());
                let err_msg = error.to_string();
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"message": err_msg})),
                )
                    .into_response()
            }
            AuthError::JsonExtractorRejection(json_rejection) => {
                let payload = json!({"message": json_rejection.to_string()});
                (json_rejection.status(), Json(payload)).into_response()
            }
            AuthError::UserNotExist => (
                StatusCode::UNAUTHORIZED,
                Json(json!({"message": "User does not exist."})),
            )
                .into_response(),
            AuthError::OutdatedGameVersion => (
                StatusCode::FORBIDDEN,
                Json(json!({"message": "Outdated game version."})),
            )
                .into_response(),
            AuthError::InvalidGameVersion => (
                StatusCode::FORBIDDEN,
                Json(json!({"message": "Invalid game version."})),
            )
                .into_response(),
            AuthError::RedisError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Internal server error."})),
            )
                .into_response(),
        }
    }
}

#[derive(Debug, Error)]
pub enum AuthErrorProvider {
    #[error(transparent)]
    Photon(#[from] PhotonAuthError),
    #[error(transparent)]
    Default(#[from] AuthError),
}

impl IntoResponse for AuthErrorProvider {
    fn into_response(self) -> Response {
        match self {
            AuthErrorProvider::Photon(err) => err.into_response(),
            AuthErrorProvider::Default(err) => err.into_response(),
        }
    }
}

#[derive(Debug, Error)]
pub enum PhotonAuthError {
    #[error("Outdated game version.")]
    OutdatedGameVersion,
    #[error("Unable to create session")]
    UnableCreateSession,
    #[error("Authentication incomplete/unsuccessful")]
    Incomplete,
    #[error("Invalid parameter input")]
    InvalidParameters,
    #[error("Invalid username/password")]
    InvalidUsernameOrPassword,
    #[error("Invalid game version")]
    InvalidGameVersion,
    #[error("Invalid API key")]
    InvalidApiKey,
    #[error("Database Error")]
    DatabaseError,
    #[error("Redis Error")]
    RedisError,
    #[error("User does not exist.")]
    UserNotExist,
    #[error("User already registered.")]
    UserAlreadyExists,
    #[error("Unauthorized access")]
    Unauthorized,
    #[error(transparent)]
    QueryExtractorRejection(#[from] QueryRejection),
    #[error(transparent)]
    JsonExtractorRejection(#[from] JsonRejection),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl IntoResponse for PhotonAuthError {
    fn into_response(self) -> Response {
        let (status, payload) = match self {
            PhotonAuthError::Incomplete => {
                let data = json!({"ResultCode": 0, "message": "Authentication incomplete."});
                (StatusCode::CONFLICT, data)
            }
            PhotonAuthError::InvalidUsernameOrPassword => {
                let data = json!({"ResultCode": 2, "message": "Authentication Failed. Wrong credentials."});
                (StatusCode::UNAUTHORIZED, data)
            }
            PhotonAuthError::InvalidParameters => {
                let data = json!({"ResultCode": 3, "message": "Invalid parameter given, try to check again."});
                (StatusCode::UNPROCESSABLE_ENTITY, data)
            }
            PhotonAuthError::QueryExtractorRejection(query_rejection) => {
                let message = query_rejection.to_string();
                let data = json!({"ResultCode": 3, "message": message});
                (query_rejection.status(), data)
            }
            PhotonAuthError::JsonExtractorRejection(json_rejection) => {
                let message = json_rejection.to_string();
                let data = json!({"ResultCode": 3, "message": message});
                (json_rejection.status(), data)
            }
            PhotonAuthError::InvalidGameVersion => {
                let data = json!({"ResultCode": 3, "message": "Invalid game version."});
                (StatusCode::UNAUTHORIZED, data)
            }
            PhotonAuthError::InvalidApiKey => {
                let data = json!({"ResultCode": 3, "message": "Invalid api key."});
                (StatusCode::UNAUTHORIZED, data)
            }
            PhotonAuthError::Other(err) => {
                let message = err.to_string();
                let data = json!({"ResultCode": 3, "message": message});
                (StatusCode::UNPROCESSABLE_ENTITY, data)
            }
            PhotonAuthError::DatabaseError => {
                let data = json!({"ResultCode": 3, "message": "Internal Server Error."});
                (StatusCode::INTERNAL_SERVER_ERROR, data)
            }
            PhotonAuthError::UnableCreateSession => {
                let data = json!({"ResultCode": 3, "message": "Internal Server Error."});
                (StatusCode::INTERNAL_SERVER_ERROR, data)
            }
            PhotonAuthError::UserNotExist => {
                let data = json!({"ResultCode": 2, "message": "User does not exist."});
                (StatusCode::FORBIDDEN, data)
            }
            PhotonAuthError::OutdatedGameVersion => {
                let data = json!({"ResultCode": 3, "message": "Outdated game version."});
                (StatusCode::FORBIDDEN, data)
            }
            PhotonAuthError::RedisError => {
                let data = json!({"ResultCode": 3, "message": "Internal Server Error."});
                (StatusCode::INTERNAL_SERVER_ERROR, data)
            }
            PhotonAuthError::UserAlreadyExists => {
                let data = json!({"ResultCode": 2, "message": "User already exists."});
                (StatusCode::CONFLICT, data)
            }
            PhotonAuthError::Unauthorized => {
                let data = json!({"ResultCode": 2, "message": "Unauthorized access."});
                (StatusCode::UNAUTHORIZED, data)
            }
        };

        (status, Json(payload)).into_response()
    }
}

impl From<AuthError> for PhotonAuthError {
    fn from(err: AuthError) -> Self {
        match err {
            AuthError::OutdatedGameVersion => PhotonAuthError::OutdatedGameVersion,
            AuthError::InvalidGameVersion => PhotonAuthError::InvalidGameVersion,
            AuthError::UserNotExist => PhotonAuthError::UserNotExist,
            AuthError::InvalidUsernameOrPassword => PhotonAuthError::InvalidUsernameOrPassword,
            AuthError::Unauthorized => PhotonAuthError::Unauthorized,
            AuthError::UnknownTokenFormat => {
                PhotonAuthError::Other(anyhow::anyhow!("Unknown token format"))
            }
            AuthError::UserRegistered => PhotonAuthError::UserAlreadyExists,
            AuthError::UnableCreateSession => PhotonAuthError::UnableCreateSession,
            AuthError::DatabaseError => PhotonAuthError::DatabaseError,
            AuthError::RedisError => PhotonAuthError::RedisError,
            AuthError::Unknown => PhotonAuthError::Other(anyhow::anyhow!("Unknown error happened")),
            AuthError::JsonExtractorRejection(json) => {
                PhotonAuthError::JsonExtractorRejection(json)
            }
            AuthError::Other(err) => PhotonAuthError::Other(err),
        }
    }
}

impl From<(JsonRejection, &AuthFormatType)> for AuthErrorProvider {
    fn from((err, format): (JsonRejection, &AuthFormatType)) -> Self {
        match format {
            AuthFormatType::Photon => {
                AuthErrorProvider::Photon(PhotonAuthError::JsonExtractorRejection(err))
            }
            AuthFormatType::Default => {
                AuthErrorProvider::Default(AuthError::JsonExtractorRejection(err))
            }
        }
    }
}

impl From<(UserServiceError, &AuthFormatType)> for AuthErrorProvider {
    fn from((err, format): (UserServiceError, &AuthFormatType)) -> Self {
        match err {
            UserServiceError::DatabaseConnectionError => match format {
                AuthFormatType::Photon => AuthErrorProvider::Photon(PhotonAuthError::DatabaseError),
                AuthFormatType::Default => AuthErrorProvider::Default(AuthError::DatabaseError),
            },
            UserServiceError::UserDoesNotExist => match format {
                AuthFormatType::Photon => AuthErrorProvider::Photon(PhotonAuthError::UserNotExist),
                AuthFormatType::Default => AuthErrorProvider::Default(AuthError::UserNotExist),
            },
            UserServiceError::PasswordNotMatch => match format {
                AuthFormatType::Photon => {
                    AuthErrorProvider::Photon(PhotonAuthError::InvalidUsernameOrPassword)
                }
                AuthFormatType::Default => {
                    AuthErrorProvider::Default(AuthError::InvalidUsernameOrPassword)
                }
            },
            UserServiceError::UnableCreateSession => match format {
                AuthFormatType::Photon => {
                    AuthErrorProvider::Photon(PhotonAuthError::UnableCreateSession)
                }
                AuthFormatType::Default => {
                    AuthErrorProvider::Default(AuthError::UnableCreateSession)
                }
            },
            UserServiceError::UserAlreadyExists => match format {
                AuthFormatType::Photon => {
                    AuthErrorProvider::Photon(PhotonAuthError::UserAlreadyExists)
                }
                AuthFormatType::Default => AuthErrorProvider::Default(AuthError::UserRegistered),
            },
            UserServiceError::UnableHashPassword => match format {
                AuthFormatType::Photon => {
                    AuthErrorProvider::Photon(PhotonAuthError::UnableCreateSession)
                }
                AuthFormatType::Default => {
                    AuthErrorProvider::Default(AuthError::UnableCreateSession)
                }
            },
            UserServiceError::RedisConnectionError => match format {
                AuthFormatType::Photon => AuthErrorProvider::Photon(PhotonAuthError::RedisError),
                AuthFormatType::Default => AuthErrorProvider::Default(AuthError::RedisError),
            },
            UserServiceError::UnauthorizedAccess => match format {
                AuthFormatType::Photon => AuthErrorProvider::Photon(PhotonAuthError::Unauthorized),
                AuthFormatType::Default => AuthErrorProvider::Default(AuthError::Unauthorized),
            },
            UserServiceError::UnableToParse => match format {
                AuthFormatType::Photon => AuthErrorProvider::Photon(PhotonAuthError::Other(
                    anyhow::anyhow!("Unable to parse incoming data."),
                )),
                AuthFormatType::Default => AuthErrorProvider::Default(AuthError::Other(
                    anyhow::anyhow!("Unable to parse incoming data."),
                )),
            },
            UserServiceError::UnexpectedError(err) => {
                tracing::error!("Got an error: {}", err.to_string());
                match format {
                    AuthFormatType::Photon => {
                        AuthErrorProvider::Photon(PhotonAuthError::Other(anyhow!("")))
                    }
                    AuthFormatType::Default => {
                        AuthErrorProvider::Default(AuthError::Other(anyhow!("")))
                    }
                }
            }
        }
    }
}

impl From<(garde::error::Report, &AuthFormatType)> for AuthErrorProvider {
    fn from((err, format): (garde::error::Report, &AuthFormatType)) -> Self {
        match format {
            AuthFormatType::Photon => {
                AuthErrorProvider::Photon(PhotonAuthError::Other(anyhow::anyhow!(err.to_string())))
            }
            AuthFormatType::Default => {
                AuthErrorProvider::Default(AuthError::Other(anyhow::anyhow!(err.to_string())))
            }
        }
    }
}

impl From<(GameServiceError, &AuthFormatType)> for AuthErrorProvider {
    fn from((err, format): (GameServiceError, &AuthFormatType)) -> Self {
        match err {
            GameServiceError::InvalidGameVersion => match format {
                AuthFormatType::Photon => {
                    AuthErrorProvider::Photon(PhotonAuthError::InvalidGameVersion)
                }
                AuthFormatType::Default => {
                    AuthErrorProvider::Default(AuthError::InvalidGameVersion)
                }
            },
            GameServiceError::OutdatedGameVersion => match format {
                AuthFormatType::Photon => {
                    AuthErrorProvider::Photon(PhotonAuthError::OutdatedGameVersion)
                }
                AuthFormatType::Default => {
                    AuthErrorProvider::Default(AuthError::OutdatedGameVersion)
                }
            },
            GameServiceError::DatabaseError => match format {
                AuthFormatType::Photon => AuthErrorProvider::Photon(PhotonAuthError::DatabaseError),
                AuthFormatType::Default => AuthErrorProvider::Default(AuthError::DatabaseError),
            },
            GameServiceError::RedisError => match format {
                AuthFormatType::Photon => AuthErrorProvider::Photon(PhotonAuthError::RedisError),
                AuthFormatType::Default => AuthErrorProvider::Default(AuthError::RedisError),
            },
        }
    }
}

impl From<UserServiceError> for AuthError {
    fn from(err: UserServiceError) -> Self {
        match err {
            UserServiceError::DatabaseConnectionError => AuthError::DatabaseError,
            UserServiceError::UserDoesNotExist => AuthError::UserNotExist,
            UserServiceError::PasswordNotMatch => AuthError::InvalidUsernameOrPassword,
            UserServiceError::UnableCreateSession => AuthError::UnableCreateSession,
            UserServiceError::UserAlreadyExists => AuthError::UserRegistered,
            UserServiceError::UnableHashPassword => AuthError::UnableCreateSession,
            UserServiceError::RedisConnectionError => AuthError::RedisError,
            UserServiceError::UnauthorizedAccess => AuthError::Unauthorized,
            UserServiceError::UnableToParse => AuthError::Other(anyhow!("Unable to parse")),
            UserServiceError::UnexpectedError(err) => {
                tracing::error!("Got an error: {}", err.to_string());
                AuthError::Unknown
            }
        }
    }
}

impl From<UserServiceError> for PhotonAuthError {
    fn from(err: UserServiceError) -> Self {
        match err {
            UserServiceError::DatabaseConnectionError => PhotonAuthError::DatabaseError,
            UserServiceError::UserDoesNotExist => PhotonAuthError::UserNotExist,
            UserServiceError::PasswordNotMatch => PhotonAuthError::InvalidUsernameOrPassword,
            UserServiceError::UnableCreateSession => PhotonAuthError::UnableCreateSession,
            UserServiceError::UserAlreadyExists => PhotonAuthError::UserAlreadyExists,
            UserServiceError::UnableHashPassword => PhotonAuthError::UnableCreateSession,
            UserServiceError::RedisConnectionError => PhotonAuthError::RedisError,
            UserServiceError::UnauthorizedAccess => PhotonAuthError::Unauthorized,
            UserServiceError::UnableToParse => PhotonAuthError::Other(anyhow!("Unable to parse")),
            UserServiceError::UnexpectedError(err) => {
                tracing::error!("Got an error: {}", err.to_string());
                PhotonAuthError::Other(anyhow!(""))
            }
        }
    }
}
