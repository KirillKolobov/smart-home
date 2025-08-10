use axum::{Router, middleware};
use sqlx::postgres::PgPoolOptions;
use utoipa::OpenApi;

use crate::{
    api_doc::ApiDoc, config::Config, middlewares::auth::auth_middleware, routes::auth::auth_router,
};

use utoipa_swagger_ui::SwaggerUi;

pub mod api_doc;
mod config;
mod db;
mod handlers;
mod middlewares;
mod models;
mod routes;
mod services;

#[derive(Clone)]
pub struct AppState {
    db: db::Database,
    config: Config,
}

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

    let app_state = AppState {
        db: db::Database::new(pool.clone()),
        config: config.clone(),
    };

    let private_routes = Router::new()
        .nest("/users", routes::users::users_router(app_state.clone()))
        .route_layer(middleware::from_fn_with_state(
            app_state.clone(),
            auth_middleware,
        ));

    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .merge(auth_router(app_state.clone()))
        .merge(private_routes);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", config.port))
        .await
        .expect("Failed to bind TCP listener");
    axum::serve(listener, app).await.unwrap();
}
