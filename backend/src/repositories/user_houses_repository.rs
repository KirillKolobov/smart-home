use async_trait::async_trait;
use mockall::automock;
use sqlx::PgPool;

use crate::{errors::Result, models::user_houses::UserHouse};

#[automock]
#[async_trait]
pub trait UserHousesRepositoryTrait {
    async fn add_house_to_user(&self, user_id: i64, house_id: i64) -> Result<UserHouse>;
    async fn user_has_access_to_house(&self, house_id: i64, user_id: i64) -> Result<bool>;
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
            user_id as i32,
            house_id as i32
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
            user_id as i32,
            house_id as i32
        )
        .fetch_optional(&self.pool)
        .await?
        .is_some();

        Ok(result)
    }
}
