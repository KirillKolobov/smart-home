use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema, Clone)]
pub struct UserHouse {
    pub user_id: i32,
    pub house_id: i32,
}
