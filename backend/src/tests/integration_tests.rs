use crate::create_app;
#[cfg(test)]
use crate::{
    models::users::User,
    tests::{create_test_config, setup_test_database},
    AppState,
};
use axum::{http::StatusCode, Router};
use axum_test::TestServer;
use rand::Rng;
use serde_json::json;
use uuid::Uuid;

async fn create_test_app() -> Result<Router, Box<dyn std::error::Error>> {
    let pool = setup_test_database().await?;
    let config = create_test_config();

    let app_state = AppState::new(crate::db::Database::new(pool), config);

    let app = create_app(app_state);

    Ok(app)
}

async fn register_unique_user(server: &TestServer) -> (String, String, String, i64) {
    // email, phone, token, user_id
    let unique_id = Uuid::new_v4();
    let email = format!("test_{}@example.com", unique_id);
    let phone = format!("111222{:04}", rand::rng().random_range(0..10000));
    let password = "password123".to_string();

    let register_payload = json!({
        "first_name": "Test",
        "last_name": "User",
        "phone": phone,
        "email": email,
        "password": password
    });

    let response = server.post("/auth/signup").json(&register_payload).await;
    assert_eq!(response.status_code(), StatusCode::CREATED);

    let login_payload = json!({
        "email": email,
        "password": password
    });
    let response = server.post("/auth/login").json(&login_payload).await;
    assert_eq!(response.status_code(), StatusCode::OK);
    let auth_response: serde_json::Value = response.json();
    let token = auth_response["token"].as_str().unwrap().to_string();
    let user_id = auth_response["user_id"].as_i64().unwrap();

    (email, phone, token, user_id)
}

#[tokio::test] // Requires test database setup
async fn test_user_registration_and_login_flow() {
    let app = create_test_app().await.expect("Failed to create test app");
    let server = TestServer::new(app).unwrap();

    let unique_id = Uuid::new_v4();
    let email = format!("john.doe_{}@example.com", unique_id);
    let phone = format!("1234567890{:04}", rand::rng().random_range(0..10000));
    let password = "password123".to_string();

    // Test user registration
    let register_payload = json!({
        "first_name": "John",
        "last_name": "Doe",
        "phone": phone,
        "email": email,
        "password": password
    });

    let response = server.post("/auth/signup").json(&register_payload).await;

    assert_eq!(response.status_code(), StatusCode::CREATED);

    let user: User = response.json();
    assert_eq!(user.first_name, "John");
    assert_eq!(user.last_name, "Doe");
    assert_eq!(user.phone, phone);
    assert_eq!(user.email, email);

    // Test user login
    let login_payload = json!({
        "email": email,
        "password": password
    });

    let response = server.post("/auth/login").json(&login_payload).await;

    assert_eq!(response.status_code(), StatusCode::OK);

    let auth_response: serde_json::Value = response.json();
    assert!(auth_response["token"].is_string());
    assert!(auth_response["user_id"].is_number());
}

#[tokio::test] // Requires test database setup
async fn test_protected_user_endpoints() {
    let app = create_test_app().await.expect("Failed to create test app");
    let server = TestServer::new(app).unwrap();

    // First, create a user and login to get token
    let (_email, _phone, token, _) = register_unique_user(&server).await;

    // Test getting user (should require auth)
    let response = server
        .get("/profile")
        .add_header("Authorization", format!("Bearer {}", token))
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);

    // Test getting user without auth (should fail)
    let response = server.get("/profile").await;

    assert_eq!(response.status_code(), StatusCode::UNAUTHORIZED);
}

#[tokio::test] // Requires test database setup
async fn test_invalid_credentials() {
    let app = create_test_app().await.expect("Failed to create test app");
    let server = TestServer::new(app).unwrap();

    // Test login with invalid credentials
    let login_payload = json!({
        "email": "nonexistent@example.com",
        "password": "wrongpassword"
    });

    let response = server.post("/auth/login").json(&login_payload).await;

    assert_eq!(response.status_code(), StatusCode::UNAUTHORIZED);
}

#[tokio::test] // Requires test database setup
async fn test_duplicate_user_registration() {
    let app = create_test_app().await.expect("Failed to create test app");
    let server = TestServer::new(app).unwrap();

    let unique_id = Uuid::new_v4();
    let email = format!("duplicate_test_{}@example.com", unique_id);
    let phone = format!("111222{:04}", rand::rng().random_range(0..10000));
    let password = "password123".to_string();

    let register_payload = json!({
        "first_name": "Test",
        "last_name": "User",
        "phone": phone,
        "email": email,
        "password": password
    });

    // First registration should succeed
    let response = server.post("/auth/signup").json(&register_payload).await;

    assert_eq!(response.status_code(), StatusCode::CREATED);

    // Second registration with same email should fail
    let response = server.post("/auth/signup").json(&register_payload).await;

    assert_eq!(response.status_code(), StatusCode::BAD_REQUEST);
}

#[tokio::test] // Requires test database setup
async fn test_invalid_input_validation() {
    let app = create_test_app().await.expect("Failed to create test app");
    let server = TestServer::new(app).unwrap();

    let unique_id = Uuid::new_v4();
    let base_email = format!("invalid_test_{}@example.com", unique_id);
    let base_phone = format!("111222{:04}", rand::rng().random_range(0..10000));

    // Test registration with invalid email
    let register_payload = json!({
        "first_name": "Test",
        "last_name": "User",
        "phone": base_phone,
        "email": "invalid-email",
        "password": "password123"
    });

    let response = server.post("/auth/signup").json(&register_payload).await;

    assert_eq!(response.status_code(), StatusCode::BAD_REQUEST);

    // Test registration with short password
    let register_payload = json!({
        "first_name": "Test",
        "last_name": "User",
        "phone": base_phone,
        "email": base_email,
        "password": "123"
    });

    let response = server.post("/auth/signup").json(&register_payload).await;

    assert_eq!(response.status_code(), StatusCode::BAD_REQUEST);

    // Test registration with short username (assuming first_name/last_name are validated)
    let register_payload = json!({
        "first_name": "a",
        "last_name": "b",
        "phone": base_phone,
        "email": base_email,
        "password": "password123"
    });

    let response = server.post("/auth/signup").json(&register_payload).await;

    assert_eq!(response.status_code(), StatusCode::BAD_REQUEST);
}
