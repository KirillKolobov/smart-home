use std::sync::Arc;

use async_trait::async_trait;
use mockall::automock;
use tracing::info;

use crate::{
    errors::Result,
    models::users::{User, UserProfile},
    repositories::UserRepositoryTrait,
};

#[automock]
#[async_trait]
pub trait UserServiceTrait {
    async fn get_user_by_id(&self, id: i64) -> Result<User>;
    async fn get_user_profile(&self, id: i64) -> Result<UserProfile>;
    async fn delete_user(&self, id: i64) -> Result<()>;
    async fn user_exists(&self, email: &str) -> Result<bool>;
}

#[derive(Clone)]
pub struct UserService {
    user_repository: Arc<dyn UserRepositoryTrait + Send + Sync>,
}

impl UserService {
    pub fn new(user_repository: Arc<dyn UserRepositoryTrait + Send + Sync>) -> Self {
        Self { user_repository }
    }
}

#[async_trait]
impl UserServiceTrait for UserService {
    async fn get_user_by_id(&self, id: i64) -> Result<User> {
        info!("Fetching user with ID: {}", id);

        let user_entity = self.user_repository.get_user_by_id(id).await?;
        let user = User::from(user_entity);

        info!("Successfully fetched user: {}", user.id);
        Ok(user)
    }

    async fn get_user_profile(&self, id: i64) -> Result<UserProfile> {
        info!("Fetching user profile with ID: {}", id);

        let user_entity = self.user_repository.get_user_by_id(id).await?;
        let profile = UserProfile::from(user_entity);

        info!("Successfully fetched user profile: {}", profile.id);
        Ok(profile)
    }

    async fn delete_user(&self, id: i64) -> Result<()> {
        info!("Attempting to delete user with ID: {}", id);

        // Check if user exists first
        self.user_repository.get_user_by_id(id).await?;

        // Delete the user
        self.user_repository.delete_user(id).await?;

        info!("Successfully deleted user with ID: {}", id);
        Ok(())
    }

    async fn user_exists(&self, email: &str) -> Result<bool> {
        self.user_repository.user_exists_by_email(email).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        errors::AppError,
        models::users::{UserEntity, UserRole},
        repositories::user_repository::MockUserRepositoryTrait,
    };
    use chrono::Utc;

    #[tokio::test]
    async fn test_get_user_by_id_success() {
        let mut mock_repo = MockUserRepositoryTrait::new();

        let user_entity = UserEntity {
            id: 1,
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            phone: "1234567890".to_string(),
            email: "test@example.com".to_string(),
            role: UserRole::User,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            last_login_at: None,
        };

        mock_repo
            .expect_get_user_by_id()
            .with(mockall::predicate::eq(1i64))
            .times(1)
            .returning(move |_| Ok(user_entity.clone()));

        let user_service = UserService::new(Arc::new(mock_repo));
        let result = user_service.get_user_by_id(1).await;

        assert!(result.is_ok());
        let user = result.unwrap();
        assert_eq!(user.id, 1);
        assert_eq!(user.first_name, "John");
        assert_eq!(user.last_name, "Doe");
        assert_eq!(user.phone, "1234567890");
        assert_eq!(user.email, "test@example.com");
    }

    #[tokio::test]
    async fn test_get_user_by_id_not_found() {
        let mut mock_repo = MockUserRepositoryTrait::new();

        mock_repo
            .expect_get_user_by_id()
            .with(mockall::predicate::eq(999i64))
            .times(1)
            .returning(|_| Err(AppError::NotFound("User not found".to_string())));

        let user_service = UserService::new(Arc::new(mock_repo));
        let result = user_service.get_user_by_id(999).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::NotFound(_) => (),
            _ => panic!("Expected NotFound error"),
        }
    }

    #[tokio::test]
    async fn test_get_user_profile_success() {
        let mut mock_repo = MockUserRepositoryTrait::new();

        let now = Utc::now();
        let user_entity = UserEntity {
            id: 1,
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            phone: "1234567890".to_string(),
            email: "test@example.com".to_string(),
            role: UserRole::Admin,
            created_at: now,
            updated_at: now,
            last_login_at: Some(now),
        };

        mock_repo
            .expect_get_user_by_id()
            .with(mockall::predicate::eq(1i64))
            .times(1)
            .returning(move |_| Ok(user_entity.clone()));

        let user_service = UserService::new(Arc::new(mock_repo));
        let result = user_service.get_user_profile(1).await;

        assert!(result.is_ok());
        let profile = result.unwrap();
        assert_eq!(profile.id, 1);
        assert_eq!(profile.first_name, "John");
        assert_eq!(profile.last_name, "Doe");
        assert_eq!(profile.phone, "1234567890");
        assert_eq!(profile.email, "test@example.com");
        assert_eq!(profile.role, UserRole::Admin);
        assert!(profile.last_login_at.is_some());
    }

    #[tokio::test]
    async fn test_delete_user_success() {
        let mut mock_repo = MockUserRepositoryTrait::new();

        let user_entity = UserEntity {
            id: 1,
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            phone: "1234567890".to_string(),
            email: "test@example.com".to_string(),
            role: UserRole::User,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            last_login_at: None,
        };

        mock_repo
            .expect_get_user_by_id()
            .with(mockall::predicate::eq(1i64))
            .times(1)
            .returning(move |_| Ok(user_entity.clone()));

        mock_repo
            .expect_delete_user()
            .with(mockall::predicate::eq(1i64))
            .times(1)
            .returning(|_| Ok(()));

        let user_service = UserService::new(Arc::new(mock_repo));
        let result = user_service.delete_user(1).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_user_not_found() {
        let mut mock_repo = MockUserRepositoryTrait::new();

        mock_repo
            .expect_get_user_by_id()
            .with(mockall::predicate::eq(999i64))
            .times(1)
            .returning(|_| Err(AppError::NotFound("User not found".to_string())));

        let user_service = UserService::new(Arc::new(mock_repo));
        let result = user_service.delete_user(999).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::NotFound(_) => (),
            _ => panic!("Expected NotFound error"),
        }
    }

    #[tokio::test]
    async fn test_user_exists_true() {
        let mut mock_repo = MockUserRepositoryTrait::new();

        mock_repo
            .expect_user_exists_by_email()
            .with(mockall::predicate::eq("existing@example.com"))
            .times(1)
            .returning(|_| Ok(true));

        let user_service = UserService::new(Arc::new(mock_repo));
        let result = user_service.user_exists("existing@example.com").await;

        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[tokio::test]
    async fn test_user_exists_false() {
        let mut mock_repo = MockUserRepositoryTrait::new();

        mock_repo
            .expect_user_exists_by_email()
            .with(mockall::predicate::eq("nonexistent@example.com"))
            .times(1)
            .returning(|_| Ok(false));

        let user_service = UserService::new(Arc::new(mock_repo));
        let result = user_service.user_exists("nonexistent@example.com").await;

        assert!(result.is_ok());
        assert!(!result.unwrap());
    }
}
