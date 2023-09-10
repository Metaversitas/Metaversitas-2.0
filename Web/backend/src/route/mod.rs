use crate::backend::AppState;
use crate::controllers::auth::{auth_router, AUTH_PATH_CONTROLLER};
use crate::controllers::health::{health_checker, HEALTH_PATH_CONTROLLER};
use crate::controllers::homepage::{homepage, HOMEPAGE_PATH_CONTROLLER};
use crate::r#const::{ENV_ENVIRONMENT, ENV_ENVIRONMENT_DEVELOPMENT, ENV_ENVIRONMENT_PRODUCTION};
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

    let auth_router = auth_router(Arc::clone(&app_state)).await;

    Router::new()
        .nest(AUTH_PATH_CONTROLLER, auth_router)
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
