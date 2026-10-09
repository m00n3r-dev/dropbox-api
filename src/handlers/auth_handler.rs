use crate::{ApiResponse, AppError, AppState, services::auth_service};
use axum::{Json, extract::State, http::StatusCode};
use serde::Deserialize;
use serde_json::json;
use validator::Validate;

// === sign up ===
#[derive(Debug, Deserialize, Validate, Default)]
#[serde(default)]
pub struct SignUpRequest {
    #[validate(length(min = 1, message = "email is required"))]
    #[validate(email(message = "invalid email address"))]
    email: String,

    #[validate(length(min = 3, message = "username must be at least 3 characters"))]
    username: String,

    #[validate(length(min = 8, message = "password must be at least 8 characters"))]
    password: String,
}

pub async fn sign_up(
    State(state): State<AppState>,
    Json(payload): Json<SignUpRequest>,
) -> Result<ApiResponse, AppError> {
    payload.validate()?;

    let user = auth_service::register(
        &state.database,
        &payload.email,
        &payload.username,
        &payload.password,
    )
    .await?;

    return Result::Ok(ApiResponse::Json(
        StatusCode::CREATED,
        json!({"message":"sign up successful","user_id":user.id}),
    ));
}
