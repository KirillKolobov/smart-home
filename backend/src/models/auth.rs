use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i64,   // User ID
    pub exp: usize, // Expiration time
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user_id: i64,
}

#[derive(Debug, FromRow)]
pub struct PasswordHash {
    pub id: i64,
    pub password_hash: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterUser {
    #[validate(length(
        min = 3,
        max = 25,
        message = "Username must be between 3 and 25 characters"
    ))]
    pub username: String,
    #[validate(email(message = "Must be a valid email address"))]
    pub email: String,
    #[validate(length(
        min = 6,
        max = 25,
        message = "Password must be between 6 and 25 characters"
    ))]
    pub password: String,
}
