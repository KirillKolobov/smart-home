use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema, Clone)]
pub struct Room {
    pub id: i64,
    pub house_id: i64,
    pub name: String,
    pub room_type: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate, Clone, PartialEq)]
pub struct NewRoom {
    #[validate(required(message = "Room name is required"), length(min = 3))]
    pub name: Option<String>,
    #[validate(required(message = "Room type is required"), length(min = 3))]
    pub room_type: Option<String>,
}
