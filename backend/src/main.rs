use axum::Router;
use sqlx::postgres::PgPoolOptions;

use crate::config::Config;

mod config;
mod db;
mod handlers;
mod models;
mod routes;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();
    let config = Config::from_env();
    let connection_string = format!(
        "postgres://{user}:{password}@{host}:{port}/{dbname}?sslmode=disable",
        user = config.db_user,
        password = config.db_pass,
        host = config.db_host,
        port = config.db_port,
        dbname = config.db_name
    );

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&connection_string)
        .await
        .expect("Failed to connect to database");

    let app = Router::new().nest(
        "/users",
        routes::users::users_router(db::Database::new(pool)),
    );

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", config.port))
        .await
        .expect("Failed to bind TCP listener");
    axum::serve(listener, app).await.unwrap();
}
