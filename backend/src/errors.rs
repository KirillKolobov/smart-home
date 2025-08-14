use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    DatabaseError(sqlx::Error),
    ValidationError(String),
    AuthenticationError(String),
    AuthorizationError(String),
    NotFound(String),
    BadRequest(String),
    InternalServerError(String),
    JwtError(jsonwebtoken::errors::Error),
    BcryptError(bcrypt::BcryptError),
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    message: String,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::DatabaseError(e) => write!(f, "Database error: {}", e),
            AppError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            AppError::AuthenticationError(msg) => write!(f, "Authentication error: {}", msg),
            AppError::AuthorizationError(msg) => write!(f, "Authorization error: {}", msg),
            AppError::NotFound(msg) => write!(f, "Not found: {}", msg),
            AppError::BadRequest(msg) => write!(f, "Bad request: {}", msg),
            AppError::InternalServerError(msg) => write!(f, "Internal server error: {}", msg),
            AppError::JwtError(e) => write!(f, "JWT error: {}", e),
            AppError::BcryptError(e) => write!(f, "Bcrypt error: {}", e),
        }
    }
}

impl std::error::Error for AppError {}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        tracing::error!("Error occurred: {}", self);

        let (status, error_type, message) = match self {
            AppError::DatabaseError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "database_error",
                "An internal database error occurred".to_string(),
            ),
            AppError::ValidationError(msg) => (StatusCode::BAD_REQUEST, "validation_error", msg),
            AppError::AuthenticationError(msg) => {
                (StatusCode::UNAUTHORIZED, "authentication_error", msg)
            }
            AppError::AuthorizationError(msg) => {
                (StatusCode::FORBIDDEN, "authorization_error", msg)
            }
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, "not_found", msg),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, "bad_request", msg),
            AppError::InternalServerError(msg) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "internal_error", msg)
            }
            AppError::JwtError(_) => (
                StatusCode::UNAUTHORIZED,
                "jwt_error",
                "Invalid or expired token".to_string(),
            ),
            AppError::BcryptError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "bcrypt_error",
                "Password processing error".to_string(),
            ),
        };

        let body = Json(ErrorResponse {
            error: error_type.to_string(),
            message: message.to_string(),
        });

        (status, body).into_response()
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => AppError::NotFound("Resource not found".to_string()),
            _ => AppError::DatabaseError(err),
        }
    }
}

impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        AppError::JwtError(err)
    }
}

impl From<bcrypt::BcryptError> for AppError {
    fn from(err: bcrypt::BcryptError) -> Self {
        AppError::BcryptError(err)
    }
}

impl From<validator::ValidationErrors> for AppError {
    fn from(err: validator::ValidationErrors) -> Self {
        AppError::ValidationError(format!("Validation failed: {}", err))
    }
}

pub type Result<T> = std::result::Result<T, AppError>;
