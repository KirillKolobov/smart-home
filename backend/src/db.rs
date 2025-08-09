use sqlx::PgPool;

use crate::models::users::{CreateUser, User};

#[derive(Clone)]
pub struct Database {
    pool: PgPool,
}

impl Database {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_user(&self, user: CreateUser) -> Result<User, sqlx::Error> {
        sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (username, email, password_hash)
            VALUES ($1, $2, $3)
            RETURNING id, username, email
            "#,
            user.username,
            user.email,
            user.password
        )
        .fetch_one(&self.pool)
        .await
    }

    pub async fn get_user(&self, id: i32) -> Result<User, sqlx::Error> {
        sqlx::query_as!(
            User,
            r#"
            SELECT id, username, email
            FROM users
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(&self.pool)
        .await
    }

    pub async fn delete_user(&self, id: i32) -> Result<(), sqlx::Error> {
        let rows_affected = sqlx::query!(
            r#"
        DELETE FROM users
        WHERE id = $1
        "#,
            id
        )
        .execute(&self.pool)
        .await?
        .rows_affected();

        if rows_affected == 0 {
            return Err(sqlx::Error::RowNotFound);
        }

        Ok(())
    }
}
