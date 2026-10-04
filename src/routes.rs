use axum::{Router, http::StatusCode, routing::get};
use serde_json::json;

use crate::ApiResponse;

pub fn create_router() -> Router {
    return Router::new().route("/health", get(health));
}

async fn health() -> ApiResponse {
    ApiResponse::Json(StatusCode::OK, json!({"status":"ok"}))
}
