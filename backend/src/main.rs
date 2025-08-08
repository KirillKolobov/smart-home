use axum::{
    Json, Router,
    http::StatusCode,
    routing::{get, post},
};
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use std::env;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();
    dotenv().expect("Failed to load .env file");
    let port = env::var("PORT").expect("PORT MUST BE DEFINED!");
    let db_host = env::var("DB_HOST").expect("DB_HOST must be defined in .env");
    let db_port = env::var("DB_PORT")
        .expect("DB_PORT must be defined in .env")
        .parse::<u16>()
        .expect("DB_PORT must be a valid port number");
    let db_name = env::var("DB_NAME").expect("DB_NAME must be defined in .env");
    let db_user = env::var("DB_USER").expect("DB_USER must be defined in .env");
    let db_pass = env::var("DB_PASSWORD").expect("DB_PASSWORD must be defined in .env");
    let connection_string = format!(
        "postgres://{user}:{password}@{host}:{port}/{dbname}?sslmode=disable",
        user = db_user,
        password = db_pass,
        host = db_host,
        port = db_port,
        dbname = db_name
    );
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&connection_string)
        .await
        .expect("Failed to connect to database");

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/users", post(create_user));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .expect("Failed to bind TCP listener");
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> String {
    env::var("DATABASE_URL").expect("URL MUST BE DEFINED!")
}

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}
