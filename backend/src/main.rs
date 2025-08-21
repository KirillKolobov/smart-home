//! Smart Home Backend - Main Application Entry Point

use anyhow::Context;
use smart_home_backend::{
    config::Config, create_app, create_database_pool, db::Database, init_tracing, run_migrations,
    AppState,
};
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    init_tracing();
    info!("Starting Smart Home Backend");

    // Load configuration
    let config = Config::from_env();
    info!("Configuration loaded successfully");

    // Create database connection pool
    let pool = create_database_pool(&config)
        .await
        .context("Failed to connect to database")?;

    info!("Database connection established");

    // Run migrations
    run_migrations(&pool)
        .await
        .context("Failed to run migrations")?;

    info!("Database migrations completed successfully");

    // Create application state
    let app_state = AppState::new(Database::new(pool), config.clone());

    // Create application router
    let app = create_app(app_state);

    // Start server
    let listener_address = format!("0.0.0.0:{}", config.port);
    info!("Starting server on {}", listener_address);

    let listener = tokio::net::TcpListener::bind(&listener_address)
        .await
        .context("Failed to bind TCP listener")?;

    info!("Smart Home Backend is running on {}", listener_address);
    info!(
        "Swagger UI available at: http://{}/swagger-ui",
        listener_address
    );

    axum::serve(listener, app).await.context("Server error")?;

    Ok(())
}
