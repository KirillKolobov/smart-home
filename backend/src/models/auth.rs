use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i64,   // User ID
    pub exp: usize, // Expiration time
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct LoginRequest {
    #[validate(email(message = "Invalid email format"))]
    #[serde(default)]
    pub email: String,
    #[serde(default)]
    pub password: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AuthResponse {
    pub token: String,
    pub user_id: i64,
}

#[derive(Debug, FromRow, Clone)]
pub struct PasswordHash {
    pub id: i64,
    pub password_hash: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct RegisterUser {
    #[validate(length(
        min = 3,
        max = 25,
        message = "First name must be between 3 and 25 characters"
    ))]
    #[serde(default)]
    pub first_name: String,
    #[validate(length(
        min = 3,
        max = 25,
        message = "Last name must be between 3 and 25 characters"
    ))]
    #[serde(default)]
    pub last_name: String,
    #[validate(length(min = 10, max = 15, message = "Invalid phone number"))]
    #[serde(default)]
    pub phone: String,
    #[validate(email(message = "Must be a valid email address"))]
    #[serde(default)]
    pub email: String,
    #[validate(length(
        min = 6,
        max = 25,
        message = "Password must be between 6 and 25 characters"
    ))]
    #[serde(default)]
    pub password: String,
}
