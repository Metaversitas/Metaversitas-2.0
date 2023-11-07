use crate::backend::AppState;
use crate::helpers::errors::auth::AuthError;
use crate::helpers::extractor::AuthenticatedUser;
use crate::model::user::{ProfileResponse, UpdateParamsUserIdentity};
use crate::service::object_storage::ObjectStorage;
use crate::service::user::UserService;
use anyhow::anyhow;
use axum::extract::multipart::MultipartRejection;
use axum::extract::{FromRef, Multipart, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use byte_unit::Byte;
use once_cell::sync::Lazy;
use serde_json::json;
use std::sync::Arc;

#[derive(Clone, FromRef)]
pub struct UserServiceRouter {
    pub user_service: Arc<UserService>,
    pub app_state: Arc<AppState>,
    pub object_storage: Arc<ObjectStorage>,
}

pub const USER_PATH_CONTROLLER: &str = "/user";
pub async fn user_router(
    app_state: Arc<AppState>,
    user_service: Arc<UserService>,
    object_storage: Arc<ObjectStorage>,
) -> Router {
    let user_service_router = UserServiceRouter {
        user_service,
        app_state,
        object_storage,
    };

    Router::new()
        .route("/profile", get(get_profile))
        .route(
            format!("/profile/{}", UPLOAD_USER_PROFILE_PATH).as_str(),
            post(change_image_user_profile),
        )
        .with_state(user_service_router)
}

async fn get_profile(
    State(_app_state): State<Arc<AppState>>,
    State(user_service): State<Arc<UserService>>,
    auth_user: Result<AuthenticatedUser, AuthError>,
) -> Result<Response, AuthError> {
    let auth_user = auth_user?;
    let result = user_service.get_profile(auth_user.user_id.as_str()).await?;
    let response = json!(ProfileResponse {
        status: true,
        data: result
    });
    Ok((StatusCode::OK, Json(response)).into_response())
}

static DEFAULT_MAX_FILE_SIZE_UPLOAD: Lazy<Byte> =
    Lazy::new(|| Byte::from_str("1.5 MB").unwrap_or(Byte::from_bytes(1500000)));
const UPLOAD_USER_PROFILE_PATH: &str = "changeUserImage";
async fn change_image_user_profile(
    State(app_state): State<Arc<AppState>>,
    State(user_service): State<Arc<UserService>>,
    is_auth_user: Result<AuthenticatedUser, AuthError>,
    files: Result<Multipart, MultipartRejection>,
) -> Result<Response, AuthError> {
    //TODO: It should check for the uploaded image file
    // So it needs to perform some image validation
    let auth_user = is_auth_user?;
    let mut files = files.map_err(|err| {
        tracing::error!("Error from multipart with an error: {}", err.to_string());
        AuthError::Other(anyhow!("Got an error: {}", err.to_string()))
    })?;
    let mut fields = vec![];
    let mut transaction = app_state.database.begin().await.map_err(|_| {
        tracing::error!("Failed to acquire a Postgres Connection from the pool");
        AuthError::Unknown
    })?;

    if let Some(file) =  files.next_field().await.map_err(|err| {
        AuthError::Other(anyhow!(
            "Error when upload, with an error: {}",
            err.to_string()
        ))
    })? {
        fields.push(file);
    }

    if fields.is_empty() {
        return Err(AuthError::Other(anyhow!(
            "There isn't any file to be upload"
        )));
    }
    if fields.len() > 1 {
        return Err(AuthError::Other(anyhow!(
            "Only 1 file that allowed to upload"
        )));
    }

    for file in fields {
        let content_type = file
            .content_type()
            .ok_or(AuthError::Other(anyhow!("Not found a file to be upload")))?;

        if content_type != "image/png"
            && content_type != "image/jpeg"
            && content_type != "image/jpg"
        {
            return Err(AuthError::Other(anyhow!(
                "Invalid content type received: {}",
                content_type
            )));
        }
        let extension_type = content_type.strip_prefix('/').unwrap_or("png").to_owned();

        let _name = file
            .file_name()
            .ok_or(AuthError::Other(anyhow!("Unable to get file name")))?
            .to_owned();
        let data = file.bytes().await.map_err(|err| {
            AuthError::Other(anyhow!(
                "Unable to get bytes from the data, with an error: {}",
                err.to_string()
            ))
        })?;
        let path = format!(
            "user-photo-profile/{}_{}.{}",
            chrono::Utc::now().format("%d-%m-%Y_%H:%M:%S"),
            auth_user.user_id,
            extension_type
        );
        if Byte::from(data.len()) > *DEFAULT_MAX_FILE_SIZE_UPLOAD {
            return Err(AuthError::Other(anyhow!(
                "File exceeded maximum upload size!"
            )));
        }
        if data.is_empty() {
            return Err(AuthError::Other(anyhow!("File is empty!")));
        }

        app_state
            .bucket
            .put_object(path.to_owned(), data.as_ref())
            .await
            .map_err(|err| {
                tracing::error!(
                    "Unable to upload file into bucket, with an error: {}",
                    err.to_string()
                );
                AuthError::Other(anyhow!("Unable to upload file into bucket"))
            })?;

        user_service
            .update_user_identity(
                &mut transaction,
                auth_user.user_id.as_str(),
                &UpdateParamsUserIdentity {
                    full_name: None,
                    gender: None,
                    photo_url: Some(path.to_owned()),
                },
            )
            .await
            .map_err(|err| {
                tracing::error!(
                    "Unable to update user identity, with an error: {}",
                    err.to_string()
                );
                AuthError::Unknown
            })?;
    }

    transaction.commit().await.map_err(|err| {
        tracing::error!(
            "Unable to commit transaction data, with an error: {}",
            err.to_string()
        );
        AuthError::Unknown
    })?;

    let response = json!({"message": "File successfully uploaded"});
    Ok((StatusCode::CREATED, Json(response)).into_response())
}
