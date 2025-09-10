#[cfg(test)]
use crate::{
    tests::{create_test_config, setup_test_database},
    AppState,
};
use axum::{http::StatusCode, Router};
use axum_test::TestServer;
use rand::Rng;
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;

use crate::create_app;
use crate::models::{
    api_tokens::{NewApiToken, PublicApiToken},
    common::ListResponse,
};

async fn create_test_app() -> Result<(Router, PgPool), Box<dyn std::error::Error>> {
    let pool = setup_test_database().await?;
    let config = create_test_config();

    let app_state = AppState::new(crate::db::Database::new(pool.clone()), config);

    let app = create_app(app_state);

    Ok((app, pool))
}

// Helper to register and login a user
async fn register_and_login_user(server: &TestServer, _: &PgPool) -> (String, i64) {
    let unique_id = Uuid::new_v4();
    let email = format!("api_token_test_{}@example.com", unique_id);
    let phone = format!("111222{:04}", rand::rng().random_range(0..10000));

    let register_payload = json!({
        "first_name": "Test",
        "last_name": "User",
        "phone": phone,
        "email": email,
        "password": "password123"
    });
    let response = server.post("/auth/signup").json(&register_payload).await;
    assert_eq!(response.status_code(), StatusCode::CREATED);

    let login_payload = json!({
        "email": email,
        "password": "password123"
    });
    let response = server.post("/auth/login").json(&login_payload).await;
    assert_eq!(response.status_code(), StatusCode::OK);
    let auth_response: serde_json::Value = response.json();
    let token = auth_response["token"].as_str().unwrap().to_string();
    let user_id = auth_response["user"]["id"].as_i64().unwrap();
    (token, user_id)
}

#[tokio::test]
async fn test_create_and_get_api_tokens() {
    let (app, pool) = create_test_app().await.expect("Failed to create test app");
    let server = TestServer::new(app).unwrap();

    let (token, _user_id) = register_and_login_user(&server, &pool).await;

    // Create a token
    let create_token_payload = json!({
        "name": "My Test Token"
    });

    let response = server
        .post("/tokens")
        .add_header("Authorization", format!("Bearer {}", token))
        .json(&create_token_payload)
        .await;

    assert_eq!(response.status_code(), StatusCode::CREATED);
    let new_token: NewApiToken = response.json();
    assert_eq!(new_token.name, "My Test Token");
    assert!(!new_token.token.is_empty());

    // Get all tokens
    let response = server
        .get("/tokens")
        .add_header("Authorization", format!("Bearer {}", token))
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);
    println!("Response text: {:?}", response.text());
    let tokens: ListResponse<PublicApiToken> = response.json();
    assert_eq!(tokens.items.len(), 1);
    assert_eq!(tokens.items[0].name, "My Test Token");
}
