use std::sync::Arc;

use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    Json,
};

use crate::{
    errors::{AppError, Result, ValidationErrorResponse},
    middlewares::validator::ValidatedJson,
    models::devices::{CreateDevice, Device, UpdateDevice},
    routes::{devices::DeviceRouterState, rooms::HouseAccess},
};

/// Create a new device
///
/// Creates a new device.
#[utoipa::path(
    post,
    path = "/devices",
    request_body = CreateDevice,
    responses(
        (status = 201, description = "Device created", body = Device),
        (status = 400, description = "Bad Request - Invalid input", body = ValidationErrorResponse),
        (status = 401, description = "Unauthorized", body = String),
        (status = 500, description = "Internal Server Error", body = String)
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "devices"
)]
pub async fn create_device(
    State(router_state): State<Arc<DeviceRouterState>>,
    ValidatedJson(new_device): ValidatedJson<CreateDevice>,
) -> Result<(StatusCode, Json<Device>)> {
    let device = router_state
        .device_service
        .create_device(new_device)
        .await?;
    Ok((StatusCode::CREATED, Json(device)))
}

/// Get device by ID
///
/// Retrieves a specific device by its ID.
#[utoipa::path(
    get,
    path = "/devices/{id}",
    params(
        ("id" = i64, Path, description = "Device ID")
    ),
    responses(
        (status = 200, description = "Device found", body = Device),
        (status = 401, description = "Unauthorized", body = String),
        (status = 404, description = "Device not found", body = String),
        (status = 500, description = "Internal Server Error", body = String)
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "devices"
)]
pub async fn get_device_by_id(
    State(router_state): State<Arc<DeviceRouterState>>,
    Path(device_id): Path<i64>,
) -> Result<Json<Device>> {
    let device = router_state
        .device_service
        .get_device_by_id(device_id)
        .await?;
    Ok(Json(device))
}

/// Update a device
///
/// Updates an existing device.
#[utoipa::path(
    put,
    path = "/devices/{id}",
    params(
        ("id" = i64, Path, description = "Device ID")
    ),
    request_body = UpdateDevice,
    responses(
        (status = 200, description = "Device updated", body = Device),
        (status = 400, description = "Bad Request - Invalid input", body = ValidationErrorResponse),
        (status = 401, description = "Unauthorized", body = String),
        (status = 500, description = "Internal Server Error", body = String)
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "devices"
)]
pub async fn update_device(
    State(router_state): State<Arc<DeviceRouterState>>,
    Path(device_id): Path<i64>,
    ValidatedJson(updated_device): ValidatedJson<UpdateDevice>,
) -> Result<Json<Device>> {
    let device = router_state
        .device_service
        .update_device(device_id, updated_device)
        .await?;
    Ok(Json(device))
}

/// Delete a device
///
/// Deletes a device by its ID.
#[utoipa::path(
    delete,
    path = "/devices/{id}",
    params(
        ("id" = i64, Path, description = "Device ID")
    ),
    responses(
        (status = 204, description = "Device deleted successfully"),
        (status = 401, description = "Unauthorized", body = String),
        (status = 404, description = "Device not found", body = String),
        (status = 500, description = "Internal Server Error", body = String)
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "devices"
)]
pub async fn delete_device(
    State(router_state): State<Arc<DeviceRouterState>>,
    Path(device_id): Path<i64>,
) -> Result<StatusCode> {
    router_state.device_service.delete_device(device_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// Get devices by room ID
///
/// Retrieves devices associated with a specific room by its ID.
#[utoipa::path(
    get,
    path = "/houses/{house_id}/rooms/{room_id}/devices",
    params(
        ("house_id" = i64, Path, description = "House ID"),
        ("room_id" = i64, Path, description = "Room ID")
    ),
    responses(
        (status = 200, description = "Devices found", body = Vec<Device>),
        (status = 401, description = "Unauthorized", body = String),
        (status = 404, description = "Room not found", body = String),
        (status = 500, description = "Internal Server Error", body = String)
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "devices"
)]
pub async fn get_devices_by_room_id(
    State(router_state): State<Arc<DeviceRouterState>>,
    Extension(user_id): Extension<i64>,
    Path((house_id, room_id)): Path<(i64, i64)>,
) -> Result<Json<Vec<Device>>> {
    router_state
        .access_control_service
        .validate_house_access(house_id, user_id)
        .await?;

    let room = router_state.room_service.get_room(room_id).await?;

    if room.house_id != house_id {
        return Err(AppError::AuthenticationError("Access denied".to_string()));
    }

    let devices = router_state
        .device_service
        .get_devices_by_room_id(room_id)
        .await?;
    Ok(Json(devices))
}

/// Get devices by house ID
///
/// Retrieves devices associated with a specific house by its ID.
#[utoipa::path(
    get,
    path = "/houses/{house_id}/devices",
    params(
        ("house_id" = i64, Path, description = "House ID")
    ),
    responses(
        (status = 200, description = "Devices found", body = Vec<Device>),
        (status = 404, description = "House not found", body = String),
        (status = 500, description = "Internal Server Error", body = String)
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "devices"
)]
pub async fn get_devices_by_house_id(
    State(router_state): State<Arc<DeviceRouterState>>,
    HouseAccess {
        house_id,
        user_id: _,
    }: HouseAccess,
) -> Result<Json<Vec<Device>>> {
    let devices = router_state
        .device_service
        .get_devices_by_house_id(house_id)
        .await?;
    Ok(Json(devices))
}
