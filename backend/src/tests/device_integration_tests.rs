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

use crate::models::devices::Device;
use crate::models::{houses, rooms};

async fn create_test_app() -> Result<(Router, PgPool), Box<dyn std::error::Error>> {
    let pool = setup_test_database().await?;
    let config = create_test_config();

    let app_state = AppState::new(crate::db::Database::new(pool.clone()), config);

    let app = Router::new()
        .merge(crate::routes::auth::auth_router(app_state.clone()))
        .nest(
            "/users",
            crate::routes::users::users_router(app_state.clone()),
        )
        .nest(
            "/houses",
            crate::routes::houses::houses_router(app_state.clone()),
        )
        .nest(
            "/houses/{house_id}/rooms",
            crate::routes::rooms::rooms_router(app_state.clone()),
        )
        .nest(
            "/devices",
            crate::routes::devices::devices_router(app_state.clone()),
        );

    Ok((app, pool))
}

// Helper to create a house
async fn create_house(server: &TestServer, token: &str, name: &str) -> houses::House {
    let create_house_payload = json!({
        "name": name,
        "address": name
    });
    let response = server
        .post("/houses")
        .add_header("Authorization", format!("Bearer {}", token))
        .json(&create_house_payload)
        .await;
    assert_eq!(response.status_code(), StatusCode::CREATED);
    response.json()
}

// Helper to create a room
async fn create_room(server: &TestServer, token: &str, house_id: i32, name: &str) -> rooms::Room {
    let create_room_payload = json!({
        "name": name,
        "room_type": "Living Room"
    });
    let response = server
        .post(&format!("/houses/{}/rooms", house_id))
        .add_header("Authorization", format!("Bearer {}", token))
        .json(&create_room_payload)
        .await;
    assert_eq!(response.status_code(), StatusCode::CREATED);
    response.json()
}

// Helper to register and login a user
async fn register_and_login_user(server: &TestServer, _: &PgPool) -> (String, i32) {
    let unique_id = Uuid::new_v4();
    let email = format!("device_test_{}@example.com", unique_id);
    let phone = format!("111222{:04}", rand::rng().random_range(0..10000));

    let register_payload = json!({
        "first_name": "Test",
        "last_name": "User",
        "phone": phone,
        "email": email,
        "password": "password123"
    });
    let response = server.post("/register").json(&register_payload).await;
    assert_eq!(response.status_code(), StatusCode::CREATED);

    let login_payload = json!({
        "email": email,
        "password": "password123"
    });
    let response = server.post("/login").json(&login_payload).await;
    assert_eq!(response.status_code(), StatusCode::OK);
    let auth_response: serde_json::Value = response.json();
    let token = auth_response["token"].as_str().unwrap().to_string();
    let user_id = auth_response["user_id"].as_i64().unwrap() as i32;
    (token, user_id)
}

#[tokio::test] // Requires test database setup
async fn test_create_device() {
    let (app, pool) = create_test_app().await.expect("Failed to create test app");
    let server = TestServer::new(app).unwrap();

    let (token, _user_id) = register_and_login_user(&server, &pool).await;
    let house = create_house(&server, &token, "Test House for Device").await;
    let room = create_room(
        &server,
        &token,
        house.id.try_into().unwrap(),
        "Test Room for Device",
    )
    .await;

    let create_device_payload = json!({
        "name": "Living Room Light",
        "device_type": "Light",
        "room_id": room.id,
        "power_consumption_w": 60,
        "is_active": true
    });

    let response = server
        .post("/devices")
        .add_header("Authorization", format!("Bearer {}", token))
        .json(&create_device_payload)
        .await;

    if response.status_code() != StatusCode::CREATED {
        eprintln!(
            "Failed to create device. Status: {}",
            response.status_code()
        );
        let body_text = response.text();
        if let Ok(json_body) = serde_json::from_str::<serde_json::Value>(&body_text) {
            eprintln!("Error Body (JSON): {:#?}", json_body);
        } else {
            eprintln!("Error Body (Text): {}", body_text);
        }
    }
    assert_eq!(response.status_code(), StatusCode::CREATED);

    let device: Device = response.json();
    assert_eq!(device.name, "Living Room Light");
    assert_eq!(device.device_type, "Light");
    assert_eq!(device.room_id, room.id as i32);
}

#[tokio::test] // Requires test database setup
async fn test_get_device_by_id() {
    let (app, pool) = create_test_app().await.expect("Failed to create test app");
    let server = TestServer::new(app).unwrap();

    let (token, _user_id) = register_and_login_user(&server, &pool).await;
    let house = create_house(&server, &token, "Test House for Get Device").await;
    let room = create_room(
        &server,
        &token,
        house.id.try_into().unwrap(),
        "Test Room for Get Device",
    )
    .await;

    let create_device_payload = json!({
        "name": "Bedroom Thermostat",
        "device_type": "Thermostat",
        "room_id": room.id,
        "power_consumption_w": 10,
        "is_active": true
    });

    let response = server
        .post("/devices")
        .add_header("Authorization", format!("Bearer {}", token))
        .json(&create_device_payload)
        .await;
    let created_device: Device = response.json();

    let response = server
        .get(&format!("/devices/{}", created_device.id))
        .add_header("Authorization", format!("Bearer {}", token))
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);

    let device: Device = response.json();
    assert_eq!(device.id, created_device.id);
    assert_eq!(device.name, "Bedroom Thermostat");
}

#[tokio::test] // Requires test database setup
async fn test_update_device() {
    let (app, pool) = create_test_app().await.expect("Failed to create test app");
    let server = TestServer::new(app).unwrap();

    let (token, _user_id) = register_and_login_user(&server, &pool).await;
    let house = create_house(&server, &token, "Test House for Update Device").await;
    let room = create_room(
        &server,
        &token,
        house.id.try_into().unwrap(),
        "Test Room for Update Device",
    )
    .await;

    let create_device_payload = json!({
        "name": "Old Device Name",
        "device_type": "SmartPlug",
        "room_id": room.id,
        "power_consumption_w": 5,
        "is_active": true
    });

    let response = server
        .post("/devices")
        .add_header("Authorization", format!("Bearer {}", token))
        .json(&create_device_payload)
        .await;
    let created_device: Device = response.json();

    let update_device_payload = json!({
        "name": "New Device Name",
        "power_consumption_w": 10,
        "is_active": false
    });

    let response = server
        .put(&format!("/devices/{}", created_device.id))
        .add_header("Authorization", format!("Bearer {}", token))
        .json(&update_device_payload)
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);

    let updated_device: Device = response.json();
    assert_eq!(updated_device.id, created_device.id);
    assert_eq!(updated_device.name, "New Device Name");
}

#[tokio::test] // Requires test database setup
async fn test_delete_device() {
    let (app, pool) = create_test_app().await.expect("Failed to create test app");
    let server = TestServer::new(app).unwrap();

    let (token, _user_id) = register_and_login_user(&server, &pool).await;
    let house = create_house(&server, &token, "Test House for Delete Device").await;
    let room = create_room(
        &server,
        &token,
        house.id.try_into().unwrap(),
        "Test Room for Delete Device",
    )
    .await;

    let create_device_payload = json!({
        "name": "Device to Delete",
        "device_type": "Sensor",
        "room_id": room.id,
        "power_consumption_w": 1,
        "is_active": true
    });

    let response = server
        .post("/devices")
        .add_header("Authorization", format!("Bearer {}", token))
        .json(&create_device_payload)
        .await;
    let created_device: Device = response.json();

    let response = server
        .delete(&format!("/devices/{}", created_device.id))
        .add_header("Authorization", format!("Bearer {}", token))
        .await;

    assert_eq!(response.status_code(), StatusCode::NO_CONTENT);

    // Try to get deleted device (should fail)
    let response = server
        .get(&format!("/devices/{}", created_device.id))
        .add_header("Authorization", format!("Bearer {}", token))
        .await;

    assert_eq!(response.status_code(), StatusCode::NOT_FOUND);
}

#[tokio::test] // Requires test database setup
async fn test_get_devices_by_room_id() {
    let (app, pool) = create_test_app().await.expect("Failed to create test app");
    let server = TestServer::new(app).unwrap();

    let (token, _user_id) = register_and_login_user(&server, &pool).await;
    let house = create_house(&server, &token, "Test House for Room Devices").await;
    let room1 = create_room(&server, &token, house.id.try_into().unwrap(), "Room 1").await;
    let room2 = create_room(&server, &token, house.id.try_into().unwrap(), "Room 2").await;

    // Create devices in Room 1
    let device1_payload = json!({
        "name": "Device A",
        "device_type": "Light",
        "room_id": room1.id,
        "power_consumption_w": 10,
        "is_active": true
    });
    server
        .post("/devices")
        .add_header("Authorization", format!("Bearer {}", token))
        .json(&device1_payload)
        .await;

    let device2_payload = json!({
        "name": "Device B",
        "device_type": "SmartPlug",
        "room_id": room1.id,
        "power_consumption_w": 5,
        "is_active": true
    });
    server
        .post("/devices")
        .add_header("Authorization", format!("Bearer {}", token))
        .json(&device2_payload)
        .await;

    // Create a device in Room 2
    let device3_payload = json!({
        "name": "Device C",
        "device_type": "Sensor",
        "room_id": room2.id,
        "power_consumption_w": 2,
        "is_active": true
    });
    server
        .post("/devices")
        .add_header("Authorization", format!("Bearer {}", token))
        .json(&device3_payload)
        .await;

    // Get devices for Room 1
    let response = server
        .get(&format!("/devices/rooms/{}/devices", room1.id))
        .add_header("Authorization", format!("Bearer {}", token))
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);
    let devices: Vec<Device> = response.json();
    assert_eq!(devices.len(), 2);
    assert!(devices.iter().any(|d| d.name == "Device A"));
    assert!(devices.iter().any(|d| d.name == "Device B"));

    // Get devices for Room 2
    let response = server
        .get(&format!("/devices/rooms/{}/devices", room2.id))
        .add_header("Authorization", format!("Bearer {}", token))
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);
    let devices: Vec<Device> = response.json();
    assert_eq!(devices.len(), 1);
    assert!(devices.iter().any(|d| d.name == "Device C"));

    // Get devices for a non-existent room
    let response = server
        .get(&format!("/devices/rooms/{}/devices", 9999))
        .add_header("Authorization", format!("Bearer {}", token))
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);
    let devices: Vec<Device> = response.json();
    assert!(devices.is_empty());
}
