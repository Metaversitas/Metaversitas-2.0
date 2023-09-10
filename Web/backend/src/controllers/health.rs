use axum::Json;
use serde_json::{json, Value};

pub const HEALTH_PATH_CONTROLLER: &str = "/health";
pub async fn health_checker() -> Result<Json<Value>, ()> {
    Ok(Json(
        json!({"status": "success", "message": "ready to serve"}),
    ))
}
