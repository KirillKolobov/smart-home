use axum::{
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Validation error: {0}")]
    ValidationError(String),
    #[error("Authentication error: {0}")]
    AuthenticationError(String),
    #[error("Authorization error: {0}")]
    AuthorizationError(String),
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Bad request: {0}")]
    BadRequest(String),
    #[error("Internal server error: {0}")]
    InternalServerError(String),
    #[error("JWT error: {0}")]
    JwtError(#[from] jsonwebtoken::errors::Error),
    #[error("Bcrypt error: {0}")]
    BcryptError(#[from] bcrypt::BcryptError),
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        tracing::error!("Error occurred: {}", self.to_string());

        let (status, message) = match self {
            AppError::DatabaseError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "An internal database error occurred".to_string(),
            ),
            AppError::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::AuthenticationError(msg) => (StatusCode::UNAUTHORIZED, msg),
            AppError::AuthorizationError(msg) => (StatusCode::FORBIDDEN, msg),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::InternalServerError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::JwtError(_) => (
                StatusCode::UNAUTHORIZED,
                "Invalid or expired token".to_string(),
            ),
            AppError::BcryptError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Password processing error".to_string(),
            ),
        };

        let body = Json(ErrorResponse {
            error: message,
        });

        (status, body).into_response()
    }
}

impl From<JsonRejection> for AppError {
    fn from(rejection: JsonRejection) -> Self {
        AppError::BadRequest(rejection.to_string())
    }
}

impl From<validator::ValidationErrors> for AppError {
    fn from(err: validator::ValidationErrors) -> Self {
        AppError::ValidationError(format!("Validation failed: {}", err))
    }
}

pub type Result<T> = std::result::Result<T, AppError>;
