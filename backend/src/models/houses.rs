use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema, Clone)]
pub struct House {
    pub id: i64,
    pub name: String,
    pub address: String,
    pub r#type: String,
    pub description: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate, PartialEq, Clone)]
pub struct NewHouse {
    #[validate(length(min = 3, message = "Name must be at least 3 characters long"))]
    #[serde(default)]
    pub name: String,
    #[validate(length(min = 3, message = "Address must be at least 3 characters long"))]
    #[serde(default)]
    pub address: String,
    #[validate(length(min = 3, message = "Type must be at least 3 characters long"))]
    #[serde(default)]
    pub r#type: String,
    #[serde(default)]
    pub description: String,
}
