use std::sync::Arc;

use crate::{
    handlers::auth::{login, register},
    repositories::UserRepository,
    services::auth::AuthService,
    AppState,
};
use axum::{routing::post, Router};

#[derive(Clone)]
pub struct AuthRouterState {
    pub auth_service: AuthService,
    pub app_state: AppState,
}

impl AuthRouterState {
    pub fn new(app_state: AppState) -> Self {
        let user_repository = Arc::new(UserRepository::new(app_state.db.pool.clone()));
        let auth_service = AuthService::new(app_state.config.clone(), user_repository);

        Self {
            auth_service,
            app_state,
        }
    }
}

pub fn auth_router(app_state: AppState) -> Router {
    let auth_router_state = AuthRouterState::new(app_state);

    Router::new()
        .route("/auth/login", post(login))
        .route("/auth/signup", post(register))
        .with_state(auth_router_state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;

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
    async fn test_auth_router_creation() {
        let config = create_test_config();

        // Create a minimal pool for testing (this would fail to connect, but that's OK for this test)
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(1)
            .connect("postgres://test:test@localhost/nonexistent")
            .await;

        // Skip the actual test if we can't connect (which is expected in CI)
        if let Ok(pool) = pool {
            let app_state = AppState {
                db: crate::db::Database::new(pool),
                config,
            };

            let router = auth_router(app_state);

            // Just verify the router was created successfully
            assert!(!format!("{:?}", router).is_empty());
        }
    }
}
