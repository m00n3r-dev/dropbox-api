use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use sqlx::PgPool;

pub mod routes;

#[derive(Clone)]
pub struct AppState {
    pub app_name: String,
    pub database: PgPool,
}

// === api response =======================================================================
enum ApiResponse {
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
enum AppError {
    NotFound(String),
    BadRequest(String),
    Unauthorized,
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized".into()),
            AppError::Internal(msg) => {
                tracing::error!("Internal error: {msg}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Something went wrong".into(),
                )
            }
        };

        (status, Json(serde_json::json!({ "error": message }))).into_response()
    }
}
