use crate::{
    errors::{Result, ValidationErrorResponse},
    middlewares::validator::ValidatedJson,
    models::{
        auth::{AuthResponse, LoginRequest, RegisterUser},
        users::User,
    },
    routes::auth::AuthRouterState,
    services::AuthServiceTrait,
};
use axum::{extract::State, http::StatusCode, Json};

/// Login endpoint
///
/// Authenticates a user with email and password, returning a JWT token on success.
#[utoipa::path(
    post,
    path = "/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Successful login", body = AuthResponse),
        (status = 400, description = "Bad Request - Invalid input", body = ValidationErrorResponse),
        (status = 401, description = "Unauthorized - Invalid credentials", body = String),
        (status = 500, description = "Internal Server Error", body = String)
    ),
    tag = "auth"
)]
pub async fn login(
    State(state): State<AuthRouterState>,
    ValidatedJson(payload): ValidatedJson<LoginRequest>,
) -> Result<Json<AuthResponse>> {
    // Attempt login
    let auth_response = state.auth_service.login(payload).await?;

    Ok(Json(auth_response))
}

/// Register endpoint
///
/// Creates a new user account with the provided information.
#[utoipa::path(
    post,
    path = "/register",
    request_body = RegisterUser,
    responses(
        (status = 201, description = "User created successfully", body = User),
        (status = 400, description = "Bad Request - Invalid input or user already exists", body = ValidationErrorResponse),
        (status = 500, description = "Internal Server Error", body = String)
    ),
    tag = "auth"
)]
pub async fn register(
    State(state): State<AuthRouterState>,
    ValidatedJson(payload): ValidatedJson<RegisterUser>,
) -> Result<(StatusCode, Json<User>)> {
    // Attempt registration
    let user = state.auth_service.register(payload).await?;

    Ok((StatusCode::CREATED, Json(user)))
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;
    use crate::{
        config::Config, errors::AppError, repositories::user_repository::MockUserRepositoryTrait,
        services::auth::AuthService,
    };
    use axum::{extract::State, http::StatusCode};
    use bcrypt::{hash, DEFAULT_COST};

    fn create_test_config() -> Config {
        Config {
            port: 3000,
            db_host: "localhost".to_string(),
            db_name: "test".to_string(),
            db_port: 5432,
            db_user: "test".to_string(),
            db_pass: "test".to_string(),
            jwt_secret: "test_secret_key_that_is_long_enough".to_string(),
            jwt_expires_in: 3600,
        }
    }

    #[tokio::test]
    async fn test_login_success() {
        let mut mock_repo = MockUserRepositoryTrait::new();
        let config = create_test_config();

        let password_hash = hash("password123", DEFAULT_COST).unwrap();
        let password_data = crate::models::auth::PasswordHash {
            id: 1,
            password_hash: password_hash.clone(),
        };

        mock_repo
            .expect_get_password_hash_by_email()
            .returning(move |_| Ok(password_data.clone()));

        mock_repo.expect_update_last_login().returning(|_| Ok(()));

        let auth_service = AuthService::new(config.clone(), Arc::new(mock_repo));
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(1)
            .connect_lazy("postgres://user:password@localhost/fake_db")
            .expect("Failed to create lazy pool");
        let state = AuthRouterState {
            auth_service,
            app_state: crate::AppState {
                db: crate::db::Database::new(pool),
                config,
            },
        };

        let login_request = LoginRequest {
            email: Some("test@example.com".to_string()),
            password: Some("password123".to_string()),
        };

        let result = login(State(state), ValidatedJson(login_request)).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_login_invalid_email() {
        let mock_repo = MockUserRepositoryTrait::new();
        let config = create_test_config();

        let auth_service = AuthService::new(config.clone(), Arc::new(mock_repo));
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(1)
            .connect_lazy("postgres://user:password@localhost/fake_db")
            .expect("Failed to create lazy pool");
        let state = AuthRouterState {
            auth_service,
            app_state: crate::AppState {
                db: crate::db::Database::new(pool),
                config,
            },
        };

        let login_request = LoginRequest {
            email: Some("invalid-email".to_string()), // Invalid email format
            password: Some("password123".to_string()),
        };

        let result = login(State(state), ValidatedJson(login_request)).await;
        assert!(result.is_err());

        match result.unwrap_err() {
            AppError::ValidationError(_) => (),
            _ => panic!("Expected ValidationError"),
        }
    }

    #[tokio::test]
    async fn test_register_success() {
        let mut mock_repo = MockUserRepositoryTrait::new();
        let config = create_test_config();

        let expected_user = User {
            id: 1,
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            phone: "1234567890".to_string(),
            email: "test@example.com".to_string(),
        };

        mock_repo.expect_find_by_email().returning(|_| Ok(None));

        mock_repo
            .expect_create_user()
            .returning(move |_| Ok(expected_user.clone()));

        let auth_service = AuthService::new(config.clone(), Arc::new(mock_repo));
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(1)
            .connect_lazy("postgres://user:password@localhost/fake_db")
            .expect("Failed to create lazy pool");
        let state = AuthRouterState {
            auth_service,
            app_state: crate::AppState {
                db: crate::db::Database::new(pool),
                config,
            },
        };

        let register_request = RegisterUser {
            first_name: Some("John".to_string()),
            last_name: Some("Doe".to_string()),
            phone: Some("1234567890".to_string()),
            email: Some("test@example.com".to_string()),
            password: Some("password123".to_string()),
        };

        let result = register(State(state), ValidatedJson(register_request)).await;
        assert!(result.is_ok());

        let (status, Json(user)) = result.unwrap();
        assert_eq!(status, StatusCode::CREATED);
        assert_eq!(user.first_name, "John");
        assert_eq!(user.last_name, "Doe");
        assert_eq!(user.phone, "1234567890");
        assert_eq!(user.email, "test@example.com");
    }

    #[tokio::test]
    async fn test_register_invalid_input() {
        let mock_repo = MockUserRepositoryTrait::new();
        let config = create_test_config();

        let auth_service = AuthService::new(config.clone(), Arc::new(mock_repo));
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(1)
            .connect_lazy("postgres://user:password@localhost/fake_db")
            .expect("Failed to create lazy pool");
        let state = AuthRouterState {
            auth_service,
            app_state: crate::AppState {
                db: crate::db::Database::new(pool),
                config,
            },
        };

        let register_request = RegisterUser {
            first_name: Some("j".to_string()),        // Too short
            last_name: Some("D".to_string()),         // Too short
            phone: Some("123".to_string()),           // Too short
            email: Some("invalid-email".to_string()), // Invalid email
            password: Some("123".to_string()),        // Too short
        };

        let result = register(State(state), ValidatedJson(register_request)).await;
        assert!(result.is_err());

        match result.unwrap_err() {
            AppError::ValidationError(_) => (),
            _ => panic!("Expected ValidationError"),
        }
    }
}
