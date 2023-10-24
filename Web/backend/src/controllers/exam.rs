use crate::backend::AppState;
use crate::helpers::errors::exam::ExamControllerError;
use crate::service::user::UserService;
use axum::extract::{FromRef, Path};
use axum::response::Response;
use axum::routing::{get, post};
use axum::Router;
use std::sync::Arc;

#[derive(Clone, FromRef)]
pub struct ExamServiceRouter {
    app_state: Arc<AppState>,
    user_service: Arc<UserService>,
}

pub const EXAM_CONTROLLER_PATH: &str = "/exam";
pub async fn exam_router(app_state: Arc<AppState>, user_service: Arc<UserService>) -> Router {
    let exam_service_router = ExamServiceRouter {
        app_state,
        user_service,
    };
    Router::new()
        .route(HOME_EXAM_PATH, post(create_exam))
        .route(
            EXAM_ID_PATH,
            get(get_exam).put(update_exam).delete(delete_exam),
        )
        .with_state(exam_service_router)
}

pub const HOME_EXAM_PATH: &str = "/";
pub const EXAM_ID_PATH: &str = "/:id";

pub async fn get_exam(Path(exam_id): Path<String>) -> Result<Response, ExamControllerError> {
    todo!()
}

pub async fn create_exam() -> Result<Response, ExamControllerError> {
    todo!()
}

pub async fn delete_exam(Path(exam_id): Path<String>) -> Result<Response, ExamControllerError> {
    todo!()
}

pub async fn update_exam(Path(exam_id): Path<String>) -> Result<Response, ExamControllerError> {
    todo!()
}
