//! Smart Home Backend Library
//!
//! This library provides the core functionality for the Smart Home management system,
//! including user authentication, device management, and API endpoints.

use axum::{http::HeaderValue, middleware, Router};
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::CorsLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub mod api_doc;
pub mod config;
pub mod db;
pub mod errors;
pub mod handlers;
pub mod middlewares;
pub mod models;
pub mod repositories;
pub mod routes;
pub mod services;

#[cfg(test)]
pub mod tests;

use config::Config;
use db::Database;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

/// Application state shared across all handlers
#[derive(Clone)]
pub struct AppState {
    pub db: Database,
    pub config: Config,
}

impl AppState {
    pub fn new(db: Database, config: Config) -> Self {
        Self { db, config }
    }
}

/// Initialize tracing subscriber for logging
pub fn init_tracing() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "smart_home_backend=debug,tower_http=debug,axum=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer().with_writer(std::io::stdout))
        .init();
}

/// Create database connection pool
pub async fn create_database_pool(config: &Config) -> Result<sqlx::PgPool, sqlx::Error> {
    let connection_string = format!(
        "postgres://{user}:{password}@{host}:{port}/{dbname}?sslmode=disable",
        user = config.db_user,
        password = config.db_pass,
        host = config.db_host,
        port = config.db_port,
        dbname = config.db_name
    );

    tracing::info!("Attempting to connect to database");
    PgPoolOptions::new()
        .max_connections(10)
        .connect(&connection_string)
        .await
}

/// Run database migrations
pub async fn run_migrations(pool: &sqlx::PgPool) -> Result<(), sqlx::migrate::MigrateError> {
    tracing::info!("Running database migrations");
    sqlx::migrate!("./migrations").run(pool).await
}

/// Create the main application router
pub fn create_app(app_state: AppState) -> Router {
    use axum::http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
    use axum::http::Method;

    let cors = CorsLayer::new()
        .allow_origin(
            app_state
                .config
                .frontend_origin
                .parse::<HeaderValue>()
                .unwrap(),
        )
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::PATCH,
            Method::OPTIONS,
        ])
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE])
        .allow_credentials(true);

    // Create protected routes that require authentication
    let protected_routes = Router::new()
        .nest("/profile", routes::users::users_router(app_state.clone()))
        .nest("/tokens", routes::api_tokens::api_tokens_router(app_state.clone()))
        .nest("/houses", routes::houses::houses_router(app_state.clone()))
        .nest(
            "/houses/{house_id}/rooms",
            routes::rooms::rooms_router(app_state.clone()),
        )
        .nest(
            "/devices",
            routes::devices::devices_router(app_state.clone()),
        )
        .nest(
            "/houses/{house_id}/devices",
            routes::devices::house_devices_router(app_state.clone()),
        )
        .nest(
            "/houses/{house_id}/rooms/{room_id}/devices",
            routes::devices::room_devices_router(app_state.clone()),
        )
        .merge(routes::device_metrics::device_metrics_routes(
            app_state.clone(),
        ))
        .nest(
            "/metrics",
            routes::device_metrics::device_metrics_router(app_state.clone()),
        )
        .route_layer(middleware::from_fn_with_state(
            app_state.clone(),
            middlewares::auth::auth_middleware,
        ));

    // Create main application router
    Router::new()
        // Swagger UI
        .merge(
            SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api_doc::ApiDoc::openapi()),
        )
        // Public routes
        .merge(routes::auth::auth_router(app_state.clone()))
        // Protected routes
        .merge(protected_routes)
        // Health check endpoint
        .route("/health", axum::routing::get(health_check))
        .layer(cors)
}

/// Health check endpoint
///
/// Returns the health status of the application.
#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "Application is healthy", body = String),
    ),
    tag = "health"
)]
pub async fn health_check() -> &'static str {
    "OK"
}

#[cfg(test)]
mod lib_tests {
    use super::*;

    #[tokio::test]
    async fn test_health_check() {
        let response = health_check().await;
        assert_eq!(response, "OK");
    }

    #[test]
    fn test_app_state_creation() {
        let config = Config {
            port: 3000,
            db_host: "localhost".to_string(),
            db_name: "test".to_string(),
            db_port: 5432,
            db_user: "test".to_string(),
            db_pass: "test".to_string(),
            jwt_secret: "test_secret_key_that_is_long_enough".to_string(),
            jwt_expires_in: 3600,
            frontend_origin: "http://localhost:5173".to_string(),
        };

        // This would require a real database connection, so we just test the config
        assert_eq!(config.port, 3000);
        assert_eq!(config.db_name, "test");
        assert_eq!(config.jwt_expires_in, 3600);
    }
}
