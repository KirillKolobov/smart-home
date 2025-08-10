use serde::Serialize;
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(Debug, Serialize, FromRow, ToSchema)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
}
