use async_trait::async_trait;
use mockall::automock;
use sqlx::PgPool;

use crate::{
    errors::{AppError, Result},
    models::devices::{CreateDevice, Device, UpdateDevice},
};

#[automock]
#[async_trait]
pub trait DeviceRepositoryTrait {
    async fn create_device(&self, new_device: CreateDevice) -> Result<Device>;
    async fn get_device_by_id(&self, id: i64) -> Result<Device>;
    async fn update_device(&self, id: i64, updated_device: UpdateDevice) -> Result<Device>;
    async fn delete_device(&self, id: i64) -> Result<()>;
    async fn get_devices_by_room_id(&self, room_id: i64) -> Result<Vec<Device>>;
    async fn get_devices_by_house_id(&self, house_id: i64) -> Result<Vec<Device>>;
}

#[derive(Clone)]
pub struct DeviceRepository {
    pool: PgPool,
}

impl DeviceRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl DeviceRepositoryTrait for DeviceRepository {
    async fn create_device(&self, new_device: CreateDevice) -> Result<Device> {
        let device = sqlx::query_as!(
            Device,
            r#"
            INSERT INTO devices (name, device_type, room_id)
            VALUES ($1, $2, $3)
            RETURNING id, name, device_type, room_id, created_at, updated_at
            "#,
            new_device.name,
            new_device.device_type,
            new_device.room_id,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(device)
    }

    async fn get_device_by_id(&self, id: i64) -> Result<Device> {
        let device = sqlx::query_as!(
            Device,
            r#"
            SELECT id, name, device_type, room_id, created_at, updated_at
            FROM devices
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => {
                AppError::NotFound(format!("Device with id {} not found", id))
            }
            _ => AppError::DatabaseError(e),
        })?;

        Ok(device)
    }

    async fn update_device(&self, id: i64, updated_device: UpdateDevice) -> Result<Device> {
        let device = sqlx::query_as!(
            Device,
            r#"
            UPDATE devices
            SET
                name = COALESCE($1, name),
                device_type = COALESCE($2, device_type),
                room_id = COALESCE($3, room_id)
            WHERE id = $4
            RETURNING id, name, device_type, room_id, created_at, updated_at
            "#,
            updated_device.name,
            updated_device.device_type,
            updated_device.room_id,
            id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => {
                AppError::NotFound(format!("Device with id {} not found", id))
            }
            _ => AppError::DatabaseError(e),
        })?;

        Ok(device)
    }

    async fn delete_device(&self, id: i64) -> Result<()> {
        let rows_affected = sqlx::query!(
            r#"
            DELETE FROM devices
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await?
        .rows_affected();

        if rows_affected == 0 {
            return Err(AppError::NotFound(format!(
                "Device with id {} not found",
                id
            )));
        }

        Ok(())
    }

    async fn get_devices_by_room_id(&self, room_id: i64) -> Result<Vec<Device>> {
        let devices = sqlx::query_as!(
            Device,
            r#"
            SELECT id, name, device_type, room_id, created_at, updated_at
            FROM devices
            WHERE room_id = $1
            ORDER BY name
            "#,
            room_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(devices)
    }

    async fn get_devices_by_house_id(&self, house_id: i64) -> Result<Vec<Device>> {
        let devices = sqlx::query_as!(
            Device,
            r#"
            SELECT id, name, device_type, room_id, created_at, updated_at
            FROM devices
            WHERE room_id IN (
                SELECT id
                FROM rooms
                WHERE house_id = $1
            )
            "#,
            house_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(devices)
    }
}
