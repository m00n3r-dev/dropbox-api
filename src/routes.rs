use axum::{Router, extract::State, http::StatusCode, routing::get};
use serde_json::json;

use crate::{ApiResponse, AppError, AppState};

pub fn create_router(state: AppState) -> Router {
    return Router::new()
        .route("/health", get(health))
        .with_state(state);
}

async fn health(State(state): State<AppState>) -> Result<ApiResponse, AppError> {
    sqlx::query("SELxECT 1")
        .execute(&state.database)
        .await
        .map_err(|_| AppError::Internal("Database connection failed".into()))?;

    Ok(ApiResponse::Json(StatusCode::OK, json!({"status": "ok"})))
}
