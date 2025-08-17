use async_trait::async_trait;
use mockall::automock;
use sqlx::PgPool;

use crate::{
    errors::{AppError, Result},
    models::houses::{House, NewHouse},
};

#[automock]
#[async_trait]
pub trait HouseRepositoryTrait {
    async fn create_house(&self, house: NewHouse) -> Result<House>;
    async fn get_house_by_id(&self, id: i64) -> Result<House>;
    async fn get_user_houses(&self, user_id: i64) -> Result<Vec<House>>;
    async fn delete_house(&self, id: i64) -> Result<()>;
}

#[derive(Clone)]
pub struct HouseRepository {
    pool: PgPool,
}

impl HouseRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl HouseRepositoryTrait for HouseRepository {
    async fn create_house(&self, house: NewHouse) -> Result<House> {
        let result = sqlx::query_as!(
            House,
            r#"
            INSERT INTO houses (name, address)
            VALUES ($1, $2)
            RETURNING id, name, address, created_at, updated_at
            "#,
            house.name,
            house.address
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }

    async fn get_house_by_id(&self, id: i64) -> Result<House> {
        let result = sqlx::query_as!(
            House,
            r#"
            SELECT id, name, address, created_at, updated_at FROM houses
            WHERE id = ($1)
            "#,
            id as i32
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }

    async fn get_user_houses(&self, user_id: i64) -> Result<Vec<House>> {
        let result = sqlx::query_as!(
            House,
            r#"
            SELECT id, name, address, created_at, updated_at
            FROM houses h
            JOIN user_houses uh ON h.id = uh.house_id
            WHERE uh.user_id = ($1)
            "#,
            user_id as i32
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(result)
    }

    async fn delete_house(&self, id: i64) -> Result<()> {
        let rows_affected = sqlx::query!("DELETE FROM houses WHERE id = $1", id as i32)
            .execute(&self.pool)
            .await?
            .rows_affected();

        if rows_affected == 0 {
            return Err(AppError::NotFound("House not found".to_string()));
        }

        Ok(())
    }
}
