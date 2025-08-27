use async_trait::async_trait;
use mockall::automock;
use sqlx::PgPool;

use crate::{
    errors::Result,
    models::{houses::House, user_houses::UserHouse},
};

#[automock]
#[async_trait]
pub trait UserHousesRepositoryTrait {
    async fn add_house_to_user(&self, user_id: i64, house_id: i64) -> Result<UserHouse>;
    async fn user_has_access_to_house(&self, house_id: i64, user_id: i64) -> Result<bool>;
    async fn get_house_by_device_id(&self, device_id: i64) -> Result<House>;
}

#[derive(Clone)]
pub struct UserHousesRepository {
    pool: PgPool,
}

impl UserHousesRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserHousesRepositoryTrait for UserHousesRepository {
    async fn add_house_to_user(&self, user_id: i64, house_id: i64) -> Result<UserHouse> {
        let result = sqlx::query_as!(
            UserHouse,
            r#"
            INSERT INTO user_houses (user_id, house_id)
            VALUES ($1, $2)
            RETURNING user_id, house_id
            "#,
            user_id,
            house_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }

    async fn user_has_access_to_house(&self, house_id: i64, user_id: i64) -> Result<bool> {
        let result = sqlx::query_as!(
            UserHouse,
            r#"
            SELECT * FROM user_houses
            WHERE user_id = $1 AND house_id = $2
            "#,
            user_id,
            house_id
        )
        .fetch_optional(&self.pool)
        .await?
        .is_some();

        Ok(result)
    }

    async fn get_house_by_device_id(&self, device_id: i64) -> Result<House> {
        let house = sqlx::query_as!(
            House,
            r#"
            SELECT h.*
            FROM houses h
            JOIN rooms r ON h.id = r.house_id
            JOIN devices d ON r.id = d.room_id
            WHERE d.id = $1
            "#,
            device_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(house)
    }
}
