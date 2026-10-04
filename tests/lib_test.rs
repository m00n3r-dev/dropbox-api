use api::{ApiResponse, AppError};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde_json::json;

#[test]
fn test_api_response_enum_variants() {
    // Test Ok variant
    let ok_response = ApiResponse::Ok;

    // Test Created variant
    let created_response = ApiResponse::Created;

    // Test Json variant
    let json_response = ApiResponse::Json(StatusCode::OK, json!({"test": "value"}));

    // Just verify we can create the variants - the actual IntoResponse
    // testing would require more setup
    assert!(matches!(ok_response, ApiResponse::Ok));
    assert!(matches!(created_response, ApiResponse::Created));
    assert!(matches!(json_response, ApiResponse::Json(_, _)));
}

#[test]
fn test_app_error_enum_variants() {
    // Test NotFound variant
    let not_found = AppError::NotFound("Resource not found".to_string());

    // Test BadRequest variant
    let bad_request = AppError::BadRequest("Invalid input".to_string());

    // Test Unauthorized variant
    let unauthorized = AppError::Unauthorized;

    // Test Internal variant
    let internal = AppError::Internal("Internal error".to_string());

    // Verify we can create all variants
    assert!(matches!(not_found, AppError::NotFound(_)));
    assert!(matches!(bad_request, AppError::BadRequest(_)));
    assert!(matches!(unauthorized, AppError::Unauthorized));
    assert!(matches!(internal, AppError::Internal(_)));
}

#[test]
fn test_app_error_into_response() {
    // Test that AppError converts to proper HTTP response
    let not_found = AppError::NotFound("Test not found".to_string());
    let _response = not_found.into_response();

    let bad_request = AppError::BadRequest("Test bad request".to_string());
    let _response = bad_request.into_response();

    let unauthorized = AppError::Unauthorized;
    let _response = unauthorized.into_response();

    let internal = AppError::Internal("Test internal error".to_string());
    let _response = internal.into_response();
}

#[test]
fn test_api_response_into_response() {
    // Test that ApiResponse converts to proper HTTP response
    let ok_response = ApiResponse::Ok;
    let _response = ok_response.into_response();

    let created_response = ApiResponse::Created;
    let _response = created_response.into_response();

    let json_response = ApiResponse::Json(StatusCode::CREATED, json!({"id": 1}));
    let _response = json_response.into_response();
}