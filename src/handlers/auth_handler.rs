use crate::{ApiResponse, AppError, AppState, services::auth_service, utils::jwt};
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

// === sign in ===
#[derive(Debug, Deserialize, Validate, Default)]
#[serde(default)]
pub struct SignInRequest {
    #[validate(length(min = 1, message = "email is required"))]
    #[validate(email(message = "invalid email address"))]
    email: String,

    #[validate(length(min = 8, message = "password must be at least 8 characters"))]
    password: String,
}

pub async fn sign_in(
    State(state): State<AppState>,
    Json(payload): Json<SignInRequest>,
) -> Result<ApiResponse, AppError> {
    payload.validate()?;

    let user = auth_service::login(&state.database, &payload.email, &payload.password).await?;

    let jwt_token = jwt::create_token(user.id, user.email, user.username)
        .map_err(|_| AppError::Internal("something went wrong!".into()))?;

    return Result::Ok(ApiResponse::Json(
        StatusCode::OK,
        json!({"message":"sign in successful","jwt_token":jwt_token}),
    ));
}
