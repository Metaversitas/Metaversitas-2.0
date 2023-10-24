use crate::backend::AppState;
use crate::helpers::errors::auth::AuthError;
use crate::helpers::extractor::AuthenticatedUserWithRole;
use crate::service::user::UserService;
use axum::extract::{FromRef, Path};
use axum::routing::{get, post};
use axum::{response::Response, Router};
use std::sync::Arc;
use crate::helpers::errors::question::QuestionControllerError;

#[derive(Clone, FromRef)]
pub struct QuestionServiceRouter {
    app_state: Arc<AppState>,
    user_service: Arc<UserService>,
}

pub const QUESTION_ROUTER_PATH: &str = "/question";
pub async fn question_router(app_state: Arc<AppState>, user_service: Arc<UserService>) -> Router {
    let question_service_router = QuestionServiceRouter {
        app_state,
        user_service,
    };

    Router::new()
        .route(HOME_QUESTION_PATH, post(create_question))
        .route(
            QUESTION_ID_PATH,
            get(get_question)
                .put(update_question)
                .delete(delete_question),
        )
        .with_state(question_service_router)
}

pub const HOME_QUESTION_PATH: &str = "/";
pub const QUESTION_ID_PATH: &str = "/:id";
pub async fn get_question(
    Path(question_id): Path<String>,
    auth_user: Result<AuthenticatedUserWithRole, AuthError>,
) -> Result<Response, QuestionControllerError> {
    todo!()
}

pub async fn update_question(
    Path(question_id): Path<String>,
    auth_user: Result<AuthenticatedUserWithRole, AuthError>,
) -> Result<Response, QuestionControllerError> {
    todo!()
}

pub async fn delete_question(
    Path(question_id): Path<String>,
    auth_user: Result<AuthenticatedUserWithRole, AuthError>,
) -> Result<Response, QuestionControllerError> {
    todo!()
}

pub async fn create_question(
    Path(question_id): Path<String>,
    auth_user: Result<AuthenticatedUserWithRole, AuthError>,
) -> Result<Response, QuestionControllerError> {
    let auth_user = auth_user?;
    todo!()
}
