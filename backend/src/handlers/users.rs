use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use crate::{
    errors::Result,
    models::users::{User, UserProfile},
    routes::users::UserRouterState,
    services::UserServiceTrait,
};

/// Get user by ID endpoint
///
/// Retrieves basic user information by user ID.
#[utoipa::path(
    get,
    path = "/users/{id}",
    params(
        ("id" = i64, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User found", body = User),
        (status = 404, description = "User not found", body = String),
        (status = 500, description = "Internal Server Error", body = String)
    ),
    tag = "users"
)]
pub async fn get_user(
    State(state): State<UserRouterState>,
    Path(user_id): Path<i64>,
) -> Result<Json<User>> {
    let user = state.user_service.get_user_by_id(user_id).await?;

    Ok(Json(user))
}

/// Get user profile by ID endpoint
///
/// Retrieves detailed user profile information by user ID.
#[utoipa::path(
    get,
    path = "/users/{id}/profile",
    params(
        ("id" = i64, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User profile found", body = UserProfile),
        (status = 404, description = "User not found", body = String),
        (status = 500, description = "Internal Server Error", body = String)
    ),
    tag = "users"
)]
pub async fn get_user_profile(
    State(state): State<UserRouterState>,
    Path(user_id): Path<i64>,
) -> Result<Json<UserProfile>> {
    let profile = state.user_service.get_user_profile(user_id).await?;

    Ok(Json(profile))
}

/// Delete user by ID endpoint
///
/// Deletes a user by their ID. This is a destructive operation.
#[utoipa::path(
    delete,
    path = "/users/{id}",
    params(
        ("id" = i64, Path, description = "User ID")
    ),
    responses(
        (status = 204, description = "User deleted successfully"),
        (status = 404, description = "User not found", body = String),
        (status = 500, description = "Internal Server Error", body = String)
    ),
    tag = "users"
)]
pub async fn delete_user(
    State(state): State<UserRouterState>,
    Path(user_id): Path<i64>,
) -> Result<StatusCode> {
    state.user_service.delete_user(user_id).await?;

    Ok(StatusCode::NO_CONTENT)
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;
    use crate::{
        errors::AppError,
        models::users::{UserEntity, UserRole},
        repositories::user_repository::MockUserRepositoryTrait,
        routes::users::UserRouterState,
        services::user_service::UserService,
    };
    use axum::extract::{Path, State};
    use chrono::Utc;

    #[tokio::test]
    async fn test_get_user_success() {
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
        let state = UserRouterState { user_service };

        let result = get_user(State(state), Path(1)).await;

        assert!(result.is_ok());
        let Json(user) = result.unwrap();
        assert_eq!(user.id, 1);
        assert_eq!(user.first_name, "John");
        assert_eq!(user.last_name, "Doe");
        assert_eq!(user.phone, "1234567890");
        assert_eq!(user.email, "test@example.com");
    }

    #[tokio::test]
    async fn test_get_user_not_found() {
        let mut mock_repo = MockUserRepositoryTrait::new();

        mock_repo
            .expect_get_user_by_id()
            .with(mockall::predicate::eq(999i64))
            .times(1)
            .returning(|_| Err(AppError::NotFound("User not found".to_string())));

        let user_service = UserService::new(Arc::new(mock_repo));
        let state = UserRouterState { user_service };

        let result = get_user(State(state), Path(999)).await;

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
        let state = UserRouterState { user_service };

        let result = get_user_profile(State(state), Path(1)).await;

        assert!(result.is_ok());
        let Json(profile) = result.unwrap();
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
        let state = UserRouterState { user_service };

        let result = delete_user(State(state), Path(1)).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), StatusCode::NO_CONTENT);
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
        let state = UserRouterState { user_service };

        let result = delete_user(State(state), Path(999)).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::NotFound(_) => (),
            _ => panic!("Expected NotFound error"),
        }
    }
}
