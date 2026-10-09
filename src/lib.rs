use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use sqlx::PgPool;
use validator::ValidationErrors;

pub mod handlers;
pub mod models;
pub mod repositories;
pub mod routes;
pub mod services;
pub mod utils;

#[derive(Clone)]
pub struct AppState {
    pub app_name: String,
    pub database: PgPool,
}

// === api response =======================================================================
pub enum ApiResponse {
    Ok,
    Created,
    Json(StatusCode, serde_json::Value),
}

impl IntoResponse for ApiResponse {
    fn into_response(self) -> Response {
        match self {
            Self::Ok => StatusCode::OK.into_response(),
            Self::Created => StatusCode::CREATED.into_response(),
            Self::Json(status, data) => (status, Json(data)).into_response(),
        }
    }
}

// === error response =======================================================================
pub enum AppError {
    NotFound(String),
    BadRequest(String),
    Unauthorized,
    Conflict(String),
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized".into()),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, msg),
            AppError::Internal(msg) => {
                tracing::error!("Internal error: {msg}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Something went wrong".into(),
                )
            }
        };

        (status, Json(serde_json::json!({ "message": message }))).into_response()
    }
}

impl From<ValidationErrors> for AppError {
    fn from(error: ValidationErrors) -> Self {
        AppError::BadRequest(error.to_string())
    }
}

impl From<argon2::password_hash::Error> for AppError {
    fn from(error: argon2::password_hash::Error) -> Self {
        tracing::error!("Database error: {error}");
        AppError::Internal("Failed to process password".into())
    }
}

impl From<sqlx::Error> for AppError {
    fn from(error: sqlx::Error) -> Self {
        tracing::error!("Database error: {error}");

        AppError::Internal("Database error".into())
    }
}
