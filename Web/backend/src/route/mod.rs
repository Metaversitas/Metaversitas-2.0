use crate::backend::AppState;
use crate::controllers::auth::{auth_router, AUTH_PATH_CONTROLLER};
use crate::controllers::classroom::{classroom_router, CLASSROOM_PATH_CONTROLLER};
use crate::controllers::health::{health_checker, HEALTH_PATH_CONTROLLER};
use crate::controllers::homepage::{homepage, HOMEPAGE_PATH_CONTROLLER};
use crate::controllers::user::{user_router, USER_PATH_CONTROLLER};
use crate::r#const::{ENV_ENVIRONMENT, ENV_ENVIRONMENT_DEVELOPMENT, ENV_ENVIRONMENT_PRODUCTION};
use crate::service::classroom::ClassroomService;
use crate::service::game::GameService;
use crate::service::student::StudentService;
use crate::service::subject::SubjectService;
use crate::service::teacher::TeacherService;
use crate::service::user::UserService;
use axum::error_handling::HandleErrorLayer;
use axum::http::StatusCode;
use axum::{routing::get, BoxError, Json, Router};
use serde_json::{json, Value};
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

struct CorsConfig;

impl CorsConfig {
    pub fn development() -> CorsLayer {
        CorsLayer::very_permissive()
    }
    pub fn production() -> CorsLayer {
        CorsLayer::new()
    }
}

pub async fn create_router(app_state: Arc<AppState>) -> Router {
    #![allow(clippy::panic)]
    let cors = match std::env::var(ENV_ENVIRONMENT) {
        Ok(env) => {
            if ENV_ENVIRONMENT_PRODUCTION == env.as_str() {
                CorsConfig::production()
            } else if ENV_ENVIRONMENT_DEVELOPMENT == env.as_str() {
                CorsConfig::development()
            } else {
                tracing::error!("Can't found the current environment mode");
                panic!()
            }
        }
        Err(_) => {
            tracing::error!("Can't found the env for environment mode");
            panic!();
        }
    };

    let service_builder = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(cors)
        .layer(HandleErrorLayer::new(handle_timeout_error))
        .timeout(std::time::Duration::from_secs(10))
        .buffer(1000)
        .concurrency_limit(1000);

    let game_service = Arc::new(GameService::new(Arc::clone(&app_state)));
    let user_service = Arc::new(UserService::new(Arc::clone(&app_state)));
    let subject_service = Arc::new(SubjectService::new(Arc::clone(&app_state)));
    let classroom_service = Arc::new(ClassroomService::new(
        Arc::clone(&app_state),
        Arc::clone(&subject_service),
    ));
    let teacher_service = Arc::new(TeacherService::new(Arc::clone(&app_state)));
    let student_service = Arc::new(StudentService::new(Arc::clone(&app_state)));

    let auth_router = auth_router(
        Arc::clone(&app_state),
        Arc::clone(&game_service),
        Arc::clone(&user_service),
    )
    .await;
    let user_router = user_router(Arc::clone(&app_state), Arc::clone(&user_service)).await;
    let classroom_router = classroom_router(
        Arc::clone(&app_state),
        Arc::clone(&classroom_service),
        Arc::clone(&user_service),
        Arc::clone(&teacher_service),
        Arc::clone(&subject_service),
        Arc::clone(&student_service),
    )
    .await;

    Router::new()
        .nest(AUTH_PATH_CONTROLLER, auth_router)
        .nest(USER_PATH_CONTROLLER, user_router)
        .nest(CLASSROOM_PATH_CONTROLLER, classroom_router)
        .route(HEALTH_PATH_CONTROLLER, get(health_checker))
        .route(HOMEPAGE_PATH_CONTROLLER, get(homepage))
        .layer(service_builder)
        .fallback(fallback)
}

async fn fallback() -> (StatusCode, Json<Value>) {
    (
        StatusCode::NOT_FOUND,
        Json(json!({"code": 404, "status": false, "message": "Where are you going?"})),
    )
}

async fn handle_timeout_error(err: BoxError) -> (StatusCode, Json<Value>) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({"status": false, "message": format!("Unhandled internal error: {}", err)})),
    )
}
