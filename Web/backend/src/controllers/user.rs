use crate::backend::AppState;
use crate::helpers::errors::auth::AuthError;
use crate::helpers::extractor::AuthenticatedUser;
use crate::model::user::ProfileResponse;
use crate::service::user::UserService;
use axum::extract::{FromRef, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Json, Router};
use serde_json::json;
use std::sync::Arc;

#[derive(Clone, FromRef)]
pub struct UserServiceRouter {
    pub user_service: Arc<UserService>,
    pub app_state: Arc<AppState>,
}

pub const USER_PATH_CONTROLLER: &str = "/user";
pub async fn user_router(app_state: Arc<AppState>, user_service: Arc<UserService>) -> Router {
    let user_service_router = UserServiceRouter {
        user_service: Arc::clone(&user_service),
        app_state: Arc::clone(&app_state),
    };

    Router::new()
        .route("/profile", get(get_profile))
        .with_state(user_service_router)
}

pub async fn get_profile(
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
