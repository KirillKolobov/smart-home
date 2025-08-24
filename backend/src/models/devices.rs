use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Device {
    pub id: i64,
    pub name: String,
    pub device_type: String,
    pub room_id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreateDevice {
    #[validate(
        required(message = "Name is required"),
        length(min = 1, message = "Name cannot be empty")
    )]
    pub name: Option<String>,
    #[validate(required(message = "Device type is required"))]
    pub device_type: Option<String>,
    #[validate(required(message = "Room is required"))]
    pub room_id: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate, ToSchema)]
pub struct UpdateDevice {
    #[validate(length(min = 1, message = "Name cannot be empty"))]
    pub name: Option<String>,
    pub device_type: Option<String>,
    pub room_id: Option<i64>,
}
