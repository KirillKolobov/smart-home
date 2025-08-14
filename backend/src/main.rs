//! Smart Home Backend - Main Application Entry Point

use smart_home_backend::{
    config::Config, create_app, create_database_pool, db::Database, init_tracing, run_migrations,
    AppState,
};
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    init_tracing();
    info!("Starting Smart Home Backend");

    // Load configuration
    let config = Config::from_env();
    info!("Configuration loaded successfully");

    // Create database connection pool
    let pool = create_database_pool(&config).await.map_err(|e| {
        error!("Failed to connect to database: {}", e);
        e
    })?;

    info!("Database connection established");

    // Run migrations
    run_migrations(&pool).await.map_err(|e| {
        error!("Failed to run migrations: {}", e);
        e
    })?;

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
        .map_err(|e| {
            error!("Failed to bind TCP listener: {}", e);
            e
        })?;

    info!("Smart Home Backend is running on {}", listener_address);
    info!(
        "Swagger UI available at: http://{}/swagger-ui",
        listener_address
    );

    axum::serve(listener, app).await.map_err(|e| {
        error!("Server error: {}", e);
        e
    })?;

    Ok(())
}
