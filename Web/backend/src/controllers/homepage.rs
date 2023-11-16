use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::{json};

pub const HOMEPAGE_PATH_CONTROLLER: &str = "/";
pub async fn homepage() -> Result<Response, ()> {
    Ok((
        StatusCode::OK,
        Json(json!({"status": true, "message": "Welcome aboard!"})),
    )
        .into_response())
}
