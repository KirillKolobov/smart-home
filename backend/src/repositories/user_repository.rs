use async_trait::async_trait;
use mockall::automock;
use sqlx::PgPool;

use crate::{
    errors::{AppError, Result},
    models::{
        auth::{PasswordHash, RegisterUser},
        users::{User, UserEntity, UserRole},
    },
};

#[automock]
#[async_trait]
pub trait UserRepositoryTrait {
    async fn create_user(&self, user: RegisterUser) -> Result<User>;
    async fn get_user_by_id(&self, id: i64) -> Result<UserEntity>;
    async fn get_user_by_email(&self, email: &str) -> Result<UserEntity>;
    async fn find_by_email(&self, email: &str) -> Result<Option<UserEntity>>;
    async fn find_by_phone(&self, phone: &str) -> Result<Option<UserEntity>>;
    async fn get_password_hash_by_email(&self, email: &str) -> Result<PasswordHash>;
    async fn delete_user(&self, id: i64) -> Result<()>;
    async fn update_last_login(&self, id: i64) -> Result<()>;
    async fn user_exists_by_email(&self, email: &str) -> Result<bool>;
}

#[derive(Clone)]
pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepositoryTrait for UserRepository {
    async fn create_user(&self, user: RegisterUser) -> Result<User> {
        let result = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (first_name, last_name, phone, email, password_hash, role)
            VALUES ($1, $2, $3, $4, $5, 'user')
            RETURNING id, first_name, last_name, phone, email
            "#,
            user.first_name.unwrap(),
            user.last_name.unwrap(),
            user.phone.unwrap(),
            user.email.unwrap(),
            user.password.unwrap()
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }

    async fn get_user_by_id(&self, id: i64) -> Result<UserEntity> {
        let result = sqlx::query_as!(
            UserEntity,
            r#"
            SELECT id, first_name, last_name, phone, email, role as "role: UserRole", created_at, updated_at, last_login_at
            FROM users
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }

    async fn get_user_by_email(&self, email: &str) -> Result<UserEntity> {
        let result = sqlx::query_as!(
            UserEntity,
            r#"
            SELECT id, first_name, last_name, phone, email, role as "role: UserRole", created_at, updated_at, last_login_at
            FROM users
            WHERE email = $1
            "#,
            email
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<UserEntity>> {
        let result = sqlx::query_as!(
            UserEntity,
            r#"
            SELECT id, first_name, last_name, phone, email, role as "role: UserRole", created_at, updated_at, last_login_at
            FROM users
            WHERE email = $1
            "#,
            email
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(result)
    }

    async fn get_password_hash_by_email(&self, email: &str) -> Result<PasswordHash> {
        let result = sqlx::query_as!(
            PasswordHash,
            "SELECT id, password_hash FROM users WHERE email = $1",
            email
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }

    async fn delete_user(&self, id: i64) -> Result<()> {
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
            return Err(AppError::NotFound("User not found".to_string()));
        }

        Ok(())
    }

    async fn update_last_login(&self, id: i64) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE users
            SET last_login_at = NOW()
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn user_exists_by_email(&self, email: &str) -> Result<bool> {
        let result = sqlx::query!(
            "SELECT EXISTS(SELECT 1 FROM users WHERE email = $1) as exists",
            email
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result.exists.unwrap_or(false))
    }

    async fn find_by_phone(&self, phone: &str) -> Result<Option<UserEntity>> {
        let result = sqlx::query_as!(
            UserEntity,
            r#"
            SELECT id, first_name, last_name, phone, email, role as "role: UserRole", created_at, updated_at, last_login_at
            FROM users
            WHERE phone = $1
            "#,
            phone
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::auth::RegisterUser;

    #[tokio::test]
    async fn test_mock_user_repository() {
        let mut mock_repo = MockUserRepositoryTrait::new();

        let test_user = RegisterUser {
            first_name: Some("John".to_string()),
            last_name: Some("Doe".to_string()),
            phone: Some("1234567890".to_string()),
            email: Some("test@example.com".to_string()),
            password: Some("hashedpassword".to_string()),
        };

        let expected_user = User {
            id: 1,
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            phone: "1234567890".to_string(),
            email: "test@example.com".to_string(),
        };

        mock_repo
            .expect_create_user()
            .times(1)
            .returning(move |_| Ok(expected_user.clone()));

        let result = mock_repo.create_user(test_user).await;
        assert!(result.is_ok());

        let user = result.unwrap();
        assert_eq!(user.first_name, "John");
        assert_eq!(user.last_name, "Doe");
        assert_eq!(user.phone, "1234567890");
        assert_eq!(user.email, "test@example.com");
    }
}
