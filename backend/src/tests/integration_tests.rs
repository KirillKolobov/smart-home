#[cfg(test)]
mod integration_tests {
    use crate::{
        models::users::User,
        tests::{create_test_config, setup_test_database},
        AppState,
    };
    use axum::{http::StatusCode, Router};
    use axum_test::TestServer;
    use serde_json::json;

    async fn create_test_app() -> Result<Router, Box<dyn std::error::Error>> {
        let pool = setup_test_database().await?;
        let config = create_test_config();

        let app_state = AppState::new(crate::db::Database::new(pool), config);

        let app = Router::new()
            .merge(crate::routes::auth::auth_router(app_state.clone()))
            .nest(
                "/users",
                crate::routes::users::users_router(app_state.clone()),
            );

        Ok(app)
    }

    #[tokio::test]
    #[ignore] // Requires test database setup
    async fn test_user_registration_and_login_flow() {
        let app = create_test_app().await.expect("Failed to create test app");
        let server = TestServer::new(app).unwrap();

        // Test user registration
        let register_payload = json!({
            "first_name": "John",
            "last_name": "Doe",
            "phone": "1234567890",
            "email": "test@example.com",
            "password": "password123"
        });

        let response = server.post("/register").json(&register_payload).await;

        assert_eq!(response.status_code(), StatusCode::CREATED);

        let user: User = response.json();
        assert_eq!(user.first_name, "John");
        assert_eq!(user.last_name, "Doe");
        assert_eq!(user.phone, "1234567890");
        assert_eq!(user.email, "test@example.com");

        // Test user login
        let login_payload = json!({
            "email": "test@example.com",
            "password": "password123"
        });

        let response = server.post("/login").json(&login_payload).await;

        assert_eq!(response.status_code(), StatusCode::OK);

        let auth_response: serde_json::Value = response.json();
        assert!(auth_response["token"].is_string());
        assert!(auth_response["user_id"].is_number());
    }

    #[tokio::test]
    #[ignore] // Requires test database setup
    async fn test_protected_user_endpoints() {
        let app = create_test_app().await.expect("Failed to create test app");
        let server = TestServer::new(app).unwrap();

        // First, create a user and login to get token
        let register_payload = json!({
            "first_name": "Test",
            "last_name": "User",
            "phone": "1112223333",
            "email": "test@example.com",
            "password": "password123"
        });

        let register_response = server.post("/register").json(&register_payload).await;

        assert_eq!(register_response.status_code(), StatusCode::CREATED);

        let login_payload = json!({
            "email": "test@example.com",
            "password": "password123"
        });

        let login_response = server.post("/login").json(&login_payload).await;

        assert_eq!(login_response.status_code(), StatusCode::OK);

        let auth_response: serde_json::Value = login_response.json();
        let token = auth_response["token"].as_str().unwrap();
        let user_id = auth_response["user_id"].as_u64().unwrap();

        // Test getting user (should require auth)
        let response = server
            .get(&format!("/users/{}", user_id))
            .add_header("Authorization", format!("Bearer {}", token))
            .await;

        assert_eq!(response.status_code(), StatusCode::OK);

        // Test getting user without auth (should fail)
        let response = server.get(&format!("/users/{}", user_id)).await;

        assert_eq!(response.status_code(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    #[ignore] // Requires test database setup
    async fn test_user_deletion() {
        let app = create_test_app().await.expect("Failed to create test app");
        let server = TestServer::new(app).unwrap();

        // Create user and login
        let register_payload = json!({
            "first_name": "Test",
            "last_name": "User",
            "phone": "1112223333",
            "email": "test@example.com",
            "password": "password123"
        });

        let register_response = server.post("/register").json(&register_payload).await;

        assert_eq!(register_response.status_code(), StatusCode::CREATED);

        let login_payload = json!({
            "email": "test@example.com",
            "password": "password123"
        });

        let login_response = server.post("/login").json(&login_payload).await;

        let auth_response: serde_json::Value = login_response.json();
        let token = auth_response["token"].as_str().unwrap();
        let user_id = auth_response["user_id"].as_u64().unwrap();

        // Delete user
        let response = server
            .delete(&format!("/users/{}", user_id))
            .add_header("Authorization", format!("Bearer {}", token))
            .await;

        assert_eq!(response.status_code(), StatusCode::NO_CONTENT);

        // Try to get deleted user (should fail)
        let response = server
            .get(&format!("/users/{}", user_id))
            .add_header("Authorization", format!("Bearer {}", token))
            .await;

        assert_eq!(response.status_code(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    #[ignore] // Requires test database setup
    async fn test_invalid_credentials() {
        let app = create_test_app().await.expect("Failed to create test app");
        let server = TestServer::new(app).unwrap();

        // Test login with invalid credentials
        let login_payload = json!({
            "email": "nonexistent@example.com",
            "password": "wrongpassword"
        });

        let response = server.post("/login").json(&login_payload).await;

        assert_eq!(response.status_code(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    #[ignore] // Requires test database setup
    async fn test_duplicate_user_registration() {
        let app = create_test_app().await.expect("Failed to create test app");
        let server = TestServer::new(app).unwrap();

        let register_payload = json!({
            "first_name": "Test",
            "last_name": "User",
            "phone": "1112223333",
            "email": "test@example.com",
            "password": "password123"
        });

        // First registration should succeed
        let response = server.post("/register").json(&register_payload).await;

        assert_eq!(response.status_code(), StatusCode::CREATED);

        // Second registration with same email should fail
        let response = server.post("/register").json(&register_payload).await;

        assert_eq!(response.status_code(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    #[ignore] // Requires test database setup
    async fn test_invalid_input_validation() {
        let app = create_test_app().await.expect("Failed to create test app");
        let server = TestServer::new(app).unwrap();

        // Test registration with invalid email
        let register_payload = json!({
            "username": "testuser",
            "email": "invalid-email",
            "password": "password123"
        });

        let response = server.post("/register").json(&register_payload).await;

        assert_eq!(response.status_code(), StatusCode::BAD_REQUEST);

        // Test registration with short password
        let register_payload = json!({
            "username": "testuser",
            "email": "test@example.com",
            "password": "123"
        });

        let response = server.post("/register").json(&register_payload).await;

        assert_eq!(response.status_code(), StatusCode::BAD_REQUEST);

        // Test registration with short username
        let register_payload = json!({
            "username": "ab",
            "email": "test@example.com",
            "password": "password123"
        });

        let response = server.post("/register").json(&register_payload).await;

        assert_eq!(response.status_code(), StatusCode::BAD_REQUEST);
    }
}
