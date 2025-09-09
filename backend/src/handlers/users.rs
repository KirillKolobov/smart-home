use axum::{Extension, Json};

use crate::{
    errors::Result, models::users::User,
};

/// Get user profile by ID endpoint
///
/// Retrieves detailed user profile information by user ID.
#[utoipa::path(
    get,
    path = "/profile",
    responses(
        (status = 200, description = "User profile found", body = User),
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
    Extension(user): Extension<User>,
) -> Result<Json<User>> {
    Ok(Json(user))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::users::{User, UserRole};
    use axum::Extension;
    use chrono::Utc;

    #[tokio::test]
    async fn test_get_user_profile_success() {
        let now = Utc::now();
        let user_entity = User {
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

        let result = get_user_profile(Extension(user_entity.clone())).await;

        assert!(result.is_ok());
        let Json(profile) = result.unwrap();
        assert_eq!(profile.id, 1);
        assert_eq!(profile.first_name, "John");
        assert_eq!(profile.email, "test@example.com");
    }
}
