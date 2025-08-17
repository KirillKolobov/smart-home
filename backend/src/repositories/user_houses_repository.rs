use async_trait::async_trait;
use mockall::automock;
use sqlx::PgPool;

use crate::{errors::Result, models::user_houses::UserHouse};

#[automock]
#[async_trait]
pub trait UserHousesRepositoryTrait {
    async fn add_house_to_user(&self, user_id: i64, house_id: i64) -> Result<UserHouse>;
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
}
