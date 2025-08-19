use async_trait::async_trait;
use mockall::automock;
use sqlx::PgPool;

use crate::{
    errors::{AppError, Result},
    models::rooms::{NewRoom, Room},
};

#[automock]
#[async_trait]
pub trait RoomsRepositoryTrait {
    async fn get_house_rooms(&self, house_id: i64) -> Result<Vec<Room>>;
    async fn create_house_room(&self, house_id: i64, room: NewRoom) -> Result<Room>;
    async fn delete_room(&self, room_id: i64) -> Result<()>;
}

#[derive(Clone)]
pub struct RoomsRepository {
    pool: PgPool,
}

impl RoomsRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl RoomsRepositoryTrait for RoomsRepository {
    async fn get_house_rooms(&self, house_id: i64) -> Result<Vec<Room>> {
        let result = sqlx::query_as!(
            Room,
            r#"
            SELECT id, house_id, name, room_type, created_at, updated_at
            FROM rooms
            WHERE house_id = ($1)
            "#,
            house_id as i32
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(result)
    }

    async fn create_house_room(&self, house_id: i64, new_room: NewRoom) -> Result<Room> {
        let result = sqlx::query_as!(
            Room,
            r#"
            INSERT INTO rooms (house_id, name, room_type)
            VALUES ($1, $2, $3)
            RETURNING id, house_id, name, room_type, created_at, updated_at
            "#,
            house_id as i32,
            new_room.name,
            new_room.room_type
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }

    async fn delete_room(&self, room_id: i64) -> Result<()> {
        let rows_affected = sqlx::query!("DELETE FROM rooms WHERE id = $1", room_id as i32)
            .execute(&self.pool)
            .await?
            .rows_affected();

        if rows_affected == 0 {
            return Err(AppError::NotFound("House not found".to_string()));
        }

        Ok(())
    }
}
