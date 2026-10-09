use crate::{ApiResponse, AppError, AppState, services::auth_service, utils::jwt::verify_token};
use axum::{
    Json,
    extract::State,
    http::{HeaderValue, StatusCode, header::SET_COOKIE},
    response::IntoResponse,
};
use axum_extra::extract::CookieJar;
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
) -> Result<axum::response::Response, AppError> {
    payload.validate()?;

    let login_data =
        auth_service::login(&state.database, &payload.email, &payload.password).await?;

    let mut response =
        ApiResponse::Json(StatusCode::OK, json!({"message":"sign in successful"})).into_response();

    let access_cookie_age = 60 * 15; // 15 minutes
    let access_cookie = format!(
        "access_token={}; HttpOnly; Secure; SameSite=None; Path=/; Max-Age={}",
        login_data.access_token, access_cookie_age
    );
    response.headers_mut().append(
        SET_COOKIE,
        HeaderValue::from_str(&access_cookie)
            .map_err(|_| AppError::Internal("something went wrong!".into()))?,
    );

    let refresh_cookie_age = 60 * 60 * 24 * 30; // 30 days
    let refresh_cookie = format!(
        "refresh_token={}; HttpOnly; Secure; SameSite=None; Path=/; Max-Age={}",
        login_data.refresh_token, refresh_cookie_age
    );
    response.headers_mut().append(
        SET_COOKIE,
        HeaderValue::from_str(&refresh_cookie)
            .map_err(|_| AppError::Internal("something went wrong!".into()))?,
    );

    Ok(response)
}

// === validate user ===
pub async fn me(jar: CookieJar) -> Result<ApiResponse, AppError> {
    let token = jar
        .get("access_token")
        .map(|cookie| cookie.value())
        .ok_or(AppError::Unauthorized)?;

    let claims = verify_token(token).map_err(|_| AppError::Unauthorized)?;

    Ok(ApiResponse::Json(StatusCode::OK, json!(claims)))
}
