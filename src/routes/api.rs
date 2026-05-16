use rustbasic_core::axum::{Router, routing::get, response::Json};
use rustbasic_core::server::AppState;
use serde_json::{json, Value};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/health", get(health_check))
        .route("/version", get(version_info))
}

async fn health_check() -> Json<Value> {
    Json(json!({
        "status": "up",
        "message": "RustBasic API is running smoothly"
    }))
}

async fn version_info() -> Json<Value> {
    Json(json!({
        "framework": "RustBasic",
        "version": "0.1.7",
        "engine": "Axum"
    }))
}
