use axum::{Router, extract::State, http::StatusCode, routing::get};
use serde_json::json;

use crate::{ApiResponse, AppError, AppState, routes::auth};

pub fn create_router(state: AppState) -> Router {
    let v1 = Router::new()
        .route("/health", get(health))
        .nest("/auth", auth::router());

    return Router::new().nest("/v1", v1).with_state(state);
}

async fn health(State(state): State<AppState>) -> Result<ApiResponse, AppError> {
    sqlx::query("SELECT 1")
        .execute(&state.database)
        .await
        .map_err(|_| AppError::Internal("Database connection failed".into()))?;

    Ok(ApiResponse::Json(StatusCode::OK, json!({"status": "ok"})))
}
