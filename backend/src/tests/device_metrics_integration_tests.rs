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
use crate::models::{device_metrics::DeviceMetric, devices, houses, rooms};

async fn create_test_app() -> Result<(Router, PgPool), Box<dyn std::error::Error>> {
    let pool = setup_test_database().await?;
    let config = create_test_config();

    let app_state = AppState::new(crate::db::Database::new(pool.clone()), config);

    let app = create_app(app_state);

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
async fn create_room(server: &TestServer, token: &str, house_id: i64, name: &str) -> rooms::Room {
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

// Helper to create a device
async fn create_device(
    server: &TestServer,
    token: &str,
    room_id: i64,
    name: &str,
) -> devices::Device {
    let create_device_payload = json!({
        "name": name,
        "device_type": "Light",
        "room_id": room_id,
    });

    let response = server
        .post("/devices")
        .add_header("Authorization", format!("Bearer {}", token))
        .json(&create_device_payload)
        .await;
    assert_eq!(response.status_code(), StatusCode::CREATED);
    response.json()
}

// Helper to create a device metric
async fn create_device_metric(
    server: &TestServer,
    token: &str,
    device_id: i64,
    metric_type: &str,
    metric_value: f64,
    unit: &str,
) -> DeviceMetric {
    let create_metric_payload = json!({
        "device_id": device_id,
        "metric_type": metric_type,
        "metric_value": metric_value,
        "unit": unit
    });
    let response = server
        .post("/metrics")
        .add_header("Authorization", format!("Bearer {}", token))
        .json(&create_metric_payload)
        .await;
    assert_eq!(response.status_code(), StatusCode::CREATED);
    response.json()
}

// Helper to register and login a user
async fn register_and_login_user(server: &TestServer) -> (String, i64) {
    let unique_id = Uuid::new_v4();
    let email = format!("device_metric_test_{}@example.com", unique_id);
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
async fn test_get_metrics_for_room() {
    let (app, _pool) = create_test_app().await.expect("Failed to create test app");
    let server = TestServer::new(app).unwrap();

    let (token, _user_id) = register_and_login_user(&server).await;
    let house = create_house(&server, &token, "Test House for Room Metrics").await;
    let room = create_room(&server, &token, house.id, "Test Room for Metrics").await;
    let device = create_device(&server, &token, room.id, "Test Device for Metrics").await;

    create_device_metric(&server, &token, device.id, "temperature", 25.5, "C").await;
    create_device_metric(&server, &token, device.id, "humidity", 45.2, "%").await;

    let response = server
        .get(&format!("/rooms/{}/metrics", room.id))
        .add_header("Authorization", format!("Bearer {}", token))
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);
    let metrics: Vec<DeviceMetric> = response.json();
    assert_eq!(metrics.len(), 2);
}

#[tokio::test]
async fn test_get_metrics_for_house() {
    let (app, _pool) = create_test_app().await.expect("Failed to create test app");
    let server = TestServer::new(app).unwrap();

    let (token, _user_id) = register_and_login_user(&server).await;
    let house = create_house(&server, &token, "Test House for House Metrics").await;
    let room1 = create_room(&server, &token, house.id, "Room 1 for House Metrics").await;
    let room2 = create_room(&server, &token, house.id, "Room 2 for House Metrics").await;
    let device1 = create_device(&server, &token, room1.id, "Device 1 for House Metrics").await;
    let device2 = create_device(&server, &token, room2.id, "Device 2 for House Metrics").await;

    create_device_metric(&server, &token, device1.id, "temperature", 22.5, "C").await;
    create_device_metric(&server, &token, device2.id, "temperature", 24.5, "C").await;

    let response = server
        .get(&format!("/houses/{}/metrics", house.id))
        .add_header("Authorization", format!("Bearer {}", token))
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);
    let metrics: Vec<DeviceMetric> = response.json();
    assert_eq!(metrics.len(), 2);
}

// #[tokio::test]
// async fn test_get_aggregated_metrics_for_room() {
//     let (app, _pool) = create_test_app().await.expect("Failed to create test app");
//     let server = TestServer::new(app).unwrap();

//     let (token, _user_id) = register_and_login_user(&server).await;
//     let house = create_house(&server, &token, "Test House for Aggregated Room Metrics").await;
//     let room = create_room(
//         &server,
//         &token,
//         house.id,
//         "Test Room for Aggregated Metrics",
//     )
//     .await;
//     let device = create_device(
//         &server,
//         &token,
//         room.id,
//         "Test Device for Aggregated Metrics",
//     )
//     .await;

//     create_device_metric(&server, &token, device.id, "temperature", 20.0, "C").await;
//     create_device_metric(&server, &token, device.id, "temperature", 30.0, "C").await;

//     let response = server
//         .get(&format!(
//             "/rooms/{}/metrics?aggregate[0][metric_type]=temperature&aggregate[0][aggregate]=Avg",
//             room.id
//         ))
//         .add_header("Authorization", format!("Bearer {}", token))
//         .await;

//     assert_eq!(response.status_code(), StatusCode::OK);
//     let metrics: Vec<AggregatedDeviceMetric> = response.json();
//     assert_eq!(metrics.len(), 1);
//     assert_eq!(metrics[0].metric_type, "temperature");
//     assert_eq!(metrics[0].metric_value, 25.0);
// }

// #[tokio::test]
// async fn test_get_aggregated_metrics_for_house() {
//     let (app, _pool) = create_test_app().await.expect("Failed to create test app");
//     let server = TestServer::new(app).unwrap();

//     let (token, _user_id) = register_and_login_user(&server).await;
//     let house = create_house(&server, &token, "Test House for Aggregated House Metrics").await;
//     let room1 = create_room(
//         &server,
//         &token,
//         house.id,
//         "Room 1 for Aggregated House Metrics",
//     )
//     .await;
//     let room2 = create_room(
//         &server,
//         &token,
//         house.id,
//         "Room 2 for Aggregated House Metrics",
//     )
//     .await;
//     let device1 = create_device(
//         &server,
//         &token,
//         room1.id,
//         "Device 1 for Aggregated House Metrics",
//     )
//     .await;
//     let device2 = create_device(
//         &server,
//         &token,
//         room2.id,
//         "Device 2 for Aggregated House Metrics",
//     )
//     .await;

//     create_device_metric(&server, &token, device1.id, "electricity", 100.0, "W").await;
//     create_device_metric(&server, &token, device2.id, "electricity", 150.0, "W").await;

//     let response = server
//         .get(&format!(
//             "/houses/{}/metrics?aggregate[0][metric_type]=electricity&aggregate[0][aggregate]=Sum",
//             house.id
//         ))
//         .add_header("Authorization", format!("Bearer {}", token))
//         .await;

//     assert_eq!(response.status_code(), StatusCode::OK);
//     let metrics: Vec<AggregatedDeviceMetric> = response.json();
//     assert_eq!(metrics.len(), 1);
//     assert_eq!(metrics[0].metric_type, "electricity");
//     assert_eq!(metrics[0].metric_value, 250.0);
// }
