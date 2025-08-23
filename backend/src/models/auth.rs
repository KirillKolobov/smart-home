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
    #[validate(
        required(message = "Email is required"),
        email(message = "Invalid email format")
    )]
    pub email: Option<String>,
    #[validate(required(message = "Password is required"))]
    pub password: Option<String>,
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
    #[validate(
        required(message = "First name is required"),
        length(
            min = 3,
            max = 25,
            message = "First name must be between 3 and 25 characters"
        )
    )]
    pub first_name: Option<String>,
    #[validate(
        required(message = "Last name is required"),
        length(
            min = 3,
            max = 25,
            message = "Last name must be between 3 and 25 characters"
        )
    )]
    pub last_name: Option<String>,
    #[validate(
        required(message = "Phone number is required"),
        length(min = 10, max = 15, message = "Invalid phone number")
    )]
    pub phone: Option<String>,
    #[validate(
        required(message = "Email is required"),
        email(message = "Must be a valid email address")
    )]
    pub email: Option<String>,
    #[validate(
        required(message = "Password is required"),
        length(
            min = 6,
            max = 25,
            message = "Password must be between 6 and 25 characters"
        )
    )]
    pub password: Option<String>,
}
