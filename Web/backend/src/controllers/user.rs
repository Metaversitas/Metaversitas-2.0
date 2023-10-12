use crate::backend::AppState;
use crate::helpers::errors::{AuthError};
use crate::helpers::extractor::AuthenticatedUser;
use crate::model::user::{ProfileResponse};
use crate::service::user::UserService;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Extension, Json, Router};
use serde_json::json;
use std::sync::Arc;

pub const USER_PATH_CONTROLLER: &str = "/user";
pub async fn user_router(app_state: Arc<AppState>, user_service: Arc<UserService>) -> Router {
    Router::new().route(
        "/profile",
        get(get_profile)
            .with_state(Arc::clone(&app_state))
            .layer(Extension(user_service)),
    )
}

pub async fn get_profile(
    auth_user: Result<AuthenticatedUser, AuthError>,
    State(_app_state): State<Arc<AppState>>,
    Extension(user_service): Extension<Arc<UserService>>,
) -> Result<Response, AuthError> {
    let auth_user = auth_user?;
    let result = user_service.get_profile(auth_user.user_id.as_str()).await?;
    let response = json!(ProfileResponse {
        status: true,
        data: result
    });
    Ok((StatusCode::OK, Json(response)).into_response())
}
