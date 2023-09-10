use axum::Json;
use serde_json::{json, Value};

pub const HOMEPAGE_PATH_CONTROLLER: &str = "/";
pub async fn homepage() -> Result<Json<Value>, ()> {
    Ok(Json(json!({"status": true, "message": "Welcome aboard!"})))
}
