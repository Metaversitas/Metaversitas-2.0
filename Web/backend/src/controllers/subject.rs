use crate::backend::AppState;
use crate::helpers::errors::auth::AuthError;
use crate::helpers::errors::subject::SubjectControllerError;
use crate::helpers::extractor::AuthenticatedUser;
use crate::service::subject::SubjectService;
use axum::extract::{FromRef, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Json, Router};
use serde_json::json;
use std::sync::Arc;

#[derive(Clone, FromRef)]
pub struct SubjectRouterService {
    pub app_state: Arc<AppState>,
    pub subject_service: Arc<SubjectService>,
}

pub const SUBJECT_PATH_CONTROLLER: &str = "/subject";
pub async fn subject_router(
    app_state: Arc<AppState>,
    subject_service: Arc<SubjectService>,
) -> Router {
    let subject_router_service = SubjectRouterService {
        app_state,
        subject_service,
    };

    Router::new()
        .route("/", get(get_available_subject))
        .with_state(subject_router_service)
}

async fn get_available_subject(
    State(app_state): State<Arc<AppState>>,
    State(subject_service): State<Arc<SubjectService>>,
    is_auth_user: Result<AuthenticatedUser, AuthError>,
) -> Result<Response, SubjectControllerError> {
    let _auth_user = is_auth_user?;

    let mut transaction = app_state.database.begin().await.map_err(|_| {
        tracing::error!("Failed to acquire a Postgres Connection from the pool");
        SubjectControllerError::Unknown
    })?;

    let list_subject = subject_service
        .get_all_subject_with_secondary(&mut transaction)
        .await
        .map_err(|err| {
            tracing::error!(
                "Failed to get list of subject, with an error: {}",
                err.to_string()
            );
            SubjectControllerError::Unknown
        })?;

    let response = json!({"data": list_subject});
    Ok((StatusCode::OK, Json(response)).into_response())
}
