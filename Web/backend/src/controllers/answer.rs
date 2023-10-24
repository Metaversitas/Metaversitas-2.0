use crate::backend::AppState;
use crate::helpers::errors::answer::AnswerControllerError;
use crate::service::user::UserService;
use axum::extract::FromRef;
use axum::response::Response;
use axum::routing::{get, post};
use axum::Router;
use std::sync::Arc;

#[derive(FromRef, Clone)]
pub struct AnswerServiceRouter {
    app_state: Arc<AppState>,
    user_service: Arc<UserService>,
}

pub const ANSWER_CONTROLLER_PATH: &str = "/answer";
pub async fn answer_router(app_state: Arc<AppState>, user_service: Arc<UserService>) -> Router {
    let answer_service_router = AnswerServiceRouter {
        app_state,
        user_service,
    };
    Router::new()
        .route(HOME_PATH, post(create_answer))
        .route(
            ID_PATH,
            get(get_answer).put(update_answer).delete(delete_answer),
        )
        .with_state(answer_service_router)
}

const HOME_PATH: &str = "/";
const ID_PATH: &str = "/:id";

pub async fn get_answer() -> Result<Response, AnswerControllerError> {
    todo!()
}

pub async fn update_answer() -> Result<Response, AnswerControllerError> {
    todo!()
}

pub async fn create_answer() -> Result<Response, AnswerControllerError> {
    todo!()
}

pub async fn delete_answer() -> Result<Response, AnswerControllerError> {
    todo!()
}
