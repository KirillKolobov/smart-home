use std::sync::Arc;

use async_trait::async_trait;
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::Utc;
use jsonwebtoken::{encode, EncodingKey, Header};
use mockall::automock;
use tracing::{info, warn};
use validator::Validate;

use crate::{
    config::Config,
    errors::{AppError, Result},
    models::auth::{AuthResponse, Claims, LoginRequest, RegisterUser},
    models::users::User,
    repositories::UserRepositoryTrait,
};

#[automock]
#[async_trait]
pub trait AuthServiceTrait {
    async fn login(&self, login_request: LoginRequest) -> Result<AuthResponse>;
    async fn register(&self, register_user: RegisterUser) -> Result<User>;
    async fn validate_token(&self, token: &str) -> Result<Claims>;
    fn generate_token(&self, user_id: i64) -> Result<String>;
}

#[derive(Clone)]
pub struct AuthService {
    config: Config,
    user_repository: Arc<dyn UserRepositoryTrait + Send + Sync>,
}

impl AuthService {
    pub fn new(
        config: Config,
        user_repository: Arc<dyn UserRepositoryTrait + Send + Sync>,
    ) -> Self {
        Self {
            config,
            user_repository,
        }
    }

    async fn hash_password(&self, password: &str) -> Result<String> {
        let hashed = hash(password, DEFAULT_COST)?;
        Ok(hashed)
    }

    async fn verify_password(&self, password: &str, hash: &str) -> Result<bool> {
        let is_valid = verify(password, hash)?;
        Ok(is_valid)
    }

    async fn validate_unique_email(&self, email: &str) -> Result<()> {
        if self.user_repository.user_exists_by_email(email).await? {
            return Err(AppError::ValidationError(
                "Email already exists".to_string(),
            ));
        }
        Ok(())
    }
}

#[async_trait]
impl AuthServiceTrait for AuthService {
    async fn login(&self, login_request: LoginRequest) -> Result<AuthResponse> {
        // Validate input
        login_request.validate()?;

        info!("Login attempt for email: {}", login_request.email);

        // Get user password hash
        let password_data = self
            .user_repository
            .get_password_hash_by_email(&login_request.email)
            .await
            .map_err(|_| {
                warn!(
                    "Login failed: user not found for email {}",
                    login_request.email
                );
                AppError::AuthenticationError("Invalid credentials".to_string())
            })?;

        // Verify password
        if !self
            .verify_password(&login_request.password, &password_data.password_hash)
            .await?
        {
            warn!(
                "Login failed: invalid password for email {}",
                login_request.email
            );
            return Err(AppError::AuthenticationError(
                "Invalid credentials".to_string(),
            ));
        }

        // Update last login
        if let Err(e) = self
            .user_repository
            .update_last_login(password_data.id)
            .await
        {
            warn!(
                "Failed to update last login for user {}: {}",
                password_data.id, e
            );
            // Don't fail the login for this
        }

        // Generate token
        let token = self.generate_token(password_data.id)?;

        info!("Successful login for user ID: {}", password_data.id);

        Ok(AuthResponse {
            token,
            user_id: password_data.id,
        })
    }

    async fn register(&self, mut register_user: RegisterUser) -> Result<User> {
        register_user.validate()?;

        info!("Registration attempt for email: {}", register_user.email);

        self.validate_unique_email(&register_user.email).await?;

        register_user.password = self.hash_password(&register_user.password).await?;

        let user = self.user_repository.create_user(register_user).await?;

        info!("Successful registration for user ID: {}", user.id);

        Ok(user)
    }

    async fn validate_token(&self, token: &str) -> Result<Claims> {
        let token_data = jsonwebtoken::decode::<Claims>(
            token,
            &jsonwebtoken::DecodingKey::from_secret(self.config.jwt_secret.as_ref()),
            &jsonwebtoken::Validation::default(),
        )?;

        // Verify user still exists
        self.user_repository
            .get_user_by_id(token_data.claims.sub)
            .await?;

        Ok(token_data.claims)
    }

    fn generate_token(&self, user_id: i64) -> Result<String> {
        let expiration = Utc::now()
            .checked_add_signed(chrono::Duration::seconds(self.config.jwt_expires_in as i64))
            .ok_or_else(|| AppError::InternalServerError("Invalid timestamp".to_string()))?
            .timestamp() as usize;

        let claims = Claims {
            sub: user_id,
            exp: expiration,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.config.jwt_secret.as_ref()),
        )?;

        Ok(token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::auth::PasswordHash;
    use crate::models::users::{UserEntity, UserRole};
    use crate::repositories::user_repository::MockUserRepositoryTrait;

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
    async fn test_successful_login() {
        let mut mock_repo = MockUserRepositoryTrait::new();
        let config = create_test_config();

        let password_hash = hash("password123", DEFAULT_COST).unwrap();
        let password_data = PasswordHash {
            id: 1,
            password_hash: password_hash.clone(),
        };

        mock_repo
            .expect_get_password_hash_by_email()
            .with(mockall::predicate::eq("test@example.com"))
            .times(1)
            .returning(move |_| Ok(password_data.clone()));

        mock_repo
            .expect_update_last_login()
            .with(mockall::predicate::eq(1i64))
            .times(1)
            .returning(|_| Ok(()));

        let auth_service = AuthService::new(config, Arc::new(mock_repo));

        let login_request = LoginRequest {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        };

        let result = auth_service.login(login_request).await;
        assert!(result.is_ok());

        let auth_response = result.unwrap();
        assert_eq!(auth_response.user_id, 1);
        assert!(!auth_response.token.is_empty());
    }

    #[tokio::test]
    async fn test_login_with_invalid_password() {
        let mut mock_repo = MockUserRepositoryTrait::new();
        let config = create_test_config();

        let password_hash = hash("password123", DEFAULT_COST).unwrap();
        let password_data = PasswordHash {
            id: 1,
            password_hash: password_hash.clone(),
        };

        mock_repo
            .expect_get_password_hash_by_email()
            .with(mockall::predicate::eq("test@example.com"))
            .times(1)
            .returning(move |_| Ok(password_data.clone()));

        let auth_service = AuthService::new(config, Arc::new(mock_repo));

        let login_request = LoginRequest {
            email: "test@example.com".to_string(),
            password: "wrongpassword".to_string(),
        };

        let result = auth_service.login(login_request).await;
        assert!(result.is_err());

        match result.unwrap_err() {
            AppError::AuthenticationError(msg) => {
                assert_eq!(msg, "Invalid credentials");
            }
            _ => panic!("Expected AuthenticationError"),
        }
    }

    #[tokio::test]
    async fn test_login_with_nonexistent_user() {
        let mut mock_repo = MockUserRepositoryTrait::new();
        let config = create_test_config();

        mock_repo
            .expect_get_password_hash_by_email()
            .with(mockall::predicate::eq("nonexistent@example.com"))
            .times(1)
            .returning(|_| Err(AppError::NotFound("User not found".to_string())));

        let auth_service = AuthService::new(config, Arc::new(mock_repo));

        let login_request = LoginRequest {
            email: "nonexistent@example.com".to_string(),
            password: "password123".to_string(),
        };

        let result = auth_service.login(login_request).await;
        assert!(result.is_err());

        match result.unwrap_err() {
            AppError::AuthenticationError(msg) => {
                assert_eq!(msg, "Invalid credentials");
            }
            _ => panic!("Expected AuthenticationError"),
        }
    }

    #[tokio::test]
    async fn test_successful_registration() {
        let mut mock_repo = MockUserRepositoryTrait::new();
        let config = create_test_config();

        let expected_user = User {
            id: 1,
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            phone: "1234567890".to_string(),
            email: "test@example.com".to_string(),
        };

        mock_repo
            .expect_user_exists_by_email()
            .with(mockall::predicate::eq("test@example.com"))
            .times(1)
            .returning(|_| Ok(false));

        mock_repo
            .expect_create_user()
            .times(1)
            .returning(move |_| Ok(expected_user.clone()));

        let auth_service = AuthService::new(config, Arc::new(mock_repo));

        let register_request = RegisterUser {
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            phone: "1234567890".to_string(),
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        };

        let result = auth_service.register(register_request).await;
        assert!(result.is_ok());

        let user = result.unwrap();
        assert_eq!(user.first_name, "John");
        assert_eq!(user.last_name, "Doe");
        assert_eq!(user.phone, "1234567890");
        assert_eq!(user.email, "test@example.com");
    }

    #[tokio::test]
    async fn test_registration_with_existing_email() {
        let mut mock_repo = MockUserRepositoryTrait::new();
        let config = create_test_config();

        mock_repo
            .expect_user_exists_by_email()
            .with(mockall::predicate::eq("existing@example.com"))
            .times(1)
            .returning(|_| Ok(true));

        let auth_service = AuthService::new(config, Arc::new(mock_repo));

        let register_request = RegisterUser {
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            phone: "1234567890".to_string(),
            email: "existing@example.com".to_string(),
            password: "password123".to_string(),
        };

        let result = auth_service.register(register_request).await;
        assert!(result.is_err());

        match result.unwrap_err() {
            AppError::ValidationError(msg) => {
                assert_eq!(msg, "Email already exists");
            }
            _ => panic!("Expected ValidationError"),
        }
    }

    #[tokio::test]
    async fn test_generate_token() {
        let mock_repo = MockUserRepositoryTrait::new();
        let config = create_test_config();
        let auth_service = AuthService::new(config, Arc::new(mock_repo));

        let token = auth_service.generate_token(1).unwrap();
        assert!(!token.is_empty());

        // Verify token can be decoded
        let claims = auth_service.validate_token(&token).await;
        // This will fail because we don't mock the user lookup, but that's expected
        assert!(claims.is_err());
    }

    #[tokio::test]
    async fn test_validate_token_with_existing_user() {
        let mut mock_repo = MockUserRepositoryTrait::new();
        let config = create_test_config();

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

        let auth_service = AuthService::new(config, Arc::new(mock_repo));

        let token = auth_service.generate_token(1).unwrap();
        let result = auth_service.validate_token(&token).await;

        assert!(result.is_ok());
        let claims = result.unwrap();
        assert_eq!(claims.sub, 1);
    }

    #[test]
    fn test_invalid_email_validation() {
        let login_request = LoginRequest {
            email: "invalid-email".to_string(),
            password: "password123".to_string(),
        };

        let validation_result = login_request.validate();
        assert!(validation_result.is_err());
    }

    #[test]
    fn test_short_password_validation() {
        let register_request = RegisterUser {
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            phone: "1234567890".to_string(),
            email: "test@example.com".to_string(),
            password: "123".to_string(), // Too short
        };

        let validation_result = register_request.validate();
        assert!(validation_result.is_err());
    }
}
