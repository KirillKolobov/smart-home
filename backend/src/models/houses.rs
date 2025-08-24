use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema, Clone)]
pub struct House {
    pub id: i64,
    pub name: String,
    pub address: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate, PartialEq, Clone)]
pub struct NewHouse {
    #[validate(required(message = "Name is required"), length(min = 3))]
    pub name: Option<String>,
    #[validate(required(message = "Address is required"), length(min = 3))]
    pub address: Option<String>,
}
