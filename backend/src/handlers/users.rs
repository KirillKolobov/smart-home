use axum::{extract::State, Extension, Json};

use crate::{
    errors::Result, models::users::UserProfile, routes::users::UserRouterState,
    services::UserServiceTrait,
};

/// Get user profile by ID endpoint
///
/// Retrieves detailed user profile information by user ID.
#[utoipa::path(
    get,
    path = "/profile",
    responses(
        (status = 200, description = "User profile found", body = UserProfile),
        (status = 401, description = "Unauthorized", body = String),
        (status = 404, description = "User not found", body = String),
        (status = 500, description = "Internal Server Error", body = String)
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "users"
)]
pub async fn get_user_profile(
    State(state): State<UserRouterState>,
    Extension(user_id): Extension<i64>,
) -> Result<Json<UserProfile>> {
    let profile = state.user_service.get_user_profile(user_id).await?;

    Ok(Json(profile))
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;
    use crate::{
        models::users::{UserEntity, UserRole},
        repositories::user_repository::MockUserRepositoryTrait,
        routes::users::UserRouterState,
        services::user_service::UserService,
    };
    use axum::extract::State;
    use chrono::Utc;

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

        let result = get_user_profile(State(state), Extension(1)).await;

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
}
