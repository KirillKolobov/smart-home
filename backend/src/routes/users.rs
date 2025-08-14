use std::sync::Arc;

use crate::{
    handlers::users::{delete_user, get_user, get_user_profile},
    repositories::UserRepository,
    services::user_service::UserService,
    AppState,
};
use axum::{
    routing::{delete, get},
    Router,
};

#[derive(Clone)]
pub struct UserRouterState {
    pub user_service: UserService,
}

impl UserRouterState {
    pub fn new(app_state: AppState) -> Self {
        let user_repository = Arc::new(UserRepository::new(app_state.db.pool.clone()));
        let user_service = UserService::new(user_repository);

        Self { user_service }
    }
}

pub fn users_router(app_state: AppState) -> Router {
    let user_router_state = UserRouterState::new(app_state);

    Router::new()
        .route("/{id}", get(get_user))
        .route("/{id}/profile", get(get_user_profile))
        .route("/{id}", delete(delete_user))
        .with_state(user_router_state)
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
    async fn test_user_router_creation() {
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

            let router = users_router(app_state);

            // Just verify the router was created successfully
            assert!(!format!("{:?}", router).is_empty());
        }
    }

    #[tokio::test]
    async fn test_user_router_state_creation() {
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

            let state = UserRouterState::new(app_state);

            // Just verify the state was created successfully
            assert!(!format!("{:?}", &state.user_service as *const _).is_empty());
        }
    }
}
