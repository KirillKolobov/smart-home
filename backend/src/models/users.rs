use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

#[derive(Debug, Serialize, FromRow)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateUser {
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
