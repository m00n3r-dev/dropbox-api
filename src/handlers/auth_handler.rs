use crate::{ApiResponse, AppError, AppState, models::user::User, utils::password::hash_password};
use axum::{Json, extract::State, http::StatusCode};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;
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

    let password_hash = hash_password(&payload.password)?;


    return Result::Ok(ApiResponse::Json(
        StatusCode::CREATED,
        json!({"message":"sign up successful","user_id":user.id}),
    ));
}
