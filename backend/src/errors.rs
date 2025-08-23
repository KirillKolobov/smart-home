use axum::{
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use thiserror::Error;
use utoipa::ToSchema;
use validator::ValidationErrors;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Validation error: {0}")]
    ValidationError(#[from] ValidationErrors),
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

#[derive(Serialize, ToSchema)]
pub struct ValidationErrorResponse {
    errors: std::collections::HashMap<String, String>,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        tracing::error!("Error occurred: {}", self.to_string());

        match self {
            AppError::ValidationError(err) => {
                let mut errors = std::collections::HashMap::new();
                for (field, field_errors) in err.field_errors() {
                    let messages: String = field_errors.iter().fold(String::new(), |mut acc, e| {
                        if !acc.is_empty() {
                            acc.push_str(", ");
                        }
                        let msg = e
                            .message
                            .as_ref()
                            .map(|m| m.to_string())
                            .unwrap_or_else(|| "Invalid value".to_string());
                        acc.push_str(&msg);
                        acc
                    });
                    errors.insert(field.to_string(), messages);
                }
                (
                    StatusCode::BAD_REQUEST,
                    Json(ValidationErrorResponse { errors }),
                )
                    .into_response()
            }
            _ => {
                let (status, message) = match self {
                    AppError::DatabaseError(_) => (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "An internal database error occurred".to_string(),
                    ),
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
                    AppError::ValidationError(_) => unreachable!(),
                };

                let body = Json(ErrorResponse { error: message });

                (status, body).into_response()
            }
        }
    }
}

impl From<JsonRejection> for AppError {
    fn from(rejection: JsonRejection) -> Self {
        AppError::BadRequest(rejection.to_string())
    }
}

pub type Result<T> = std::result::Result<T, AppError>;
