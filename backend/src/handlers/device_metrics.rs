use std::sync::Arc;

use axum::{
    extract::{Extension, Path, Query, State},
    http::StatusCode,
    Json,
};

use crate::{
    errors::Result,
    middlewares::validator::ValidatedJson,
    models::{
        device_metrics::{CreateDeviceMetric, DeviceMetric, DeviceMetricFilters},
        users::User,
    },
    routes::device_metrics::DeviceMetricsRouterState,
};

/// Create a new device metric
///
/// Creates a new device metric.
#[utoipa::path(
    post,
    path = "/metrics",
    request_body = CreateDeviceMetric,
    responses(
        (status = 201, description = "Device metric created", body = DeviceMetric),
        (status = 400, description = "Bad Request - Invalid input"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal Server Error")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "device_metrics"
)]
pub async fn create_metric(
    State(router_state): State<Arc<DeviceMetricsRouterState>>,
    Extension(user): Extension<User>,
    ValidatedJson(new_metric): ValidatedJson<CreateDeviceMetric>,
) -> Result<(StatusCode, Json<DeviceMetric>)> {
    {
        let metric = router_state
            .device_metrics_service
            .create_metric(user.id, new_metric)
            .await?;
        Ok((StatusCode::CREATED, Json(metric)))
    }
}

/// Get device metrics
///
/// Retrieves device metrics with optional filters.
#[utoipa::path(
    get,
    path = "/devices/{device_id}/metrics",
    params(
        ("device_id" = i64, Path, description = "Device ID"),
        DeviceMetricFilters
    ),
    responses(
        (status = 200, description = "Device metrics found", body = Vec<DeviceMetric>),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Device not found"),
        (status = 500, description = "Internal Server Error")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "device_metrics"
)]
pub async fn get_metrics(
    State(router_state): State<Arc<DeviceMetricsRouterState>>,
    Extension(user): Extension<User>,
    Path(device_id): Path<i64>,
    Query(filters): Query<DeviceMetricFilters>,
) -> Result<Json<Vec<DeviceMetric>>> {
    {
        let metrics = router_state
            .device_metrics_service
            .get_metrics(user.id, device_id, filters)
            .await?;
        Ok(Json(metrics))
    }
}

/// Get device metrics for a room
///
/// Retrieves device metrics for a room with optional filters.
#[utoipa::path(
    get,
    path = "/rooms/{room_id}/metrics",
    params(
        ("room_id" = i64, Path, description = "Room ID"),
        DeviceMetricFilters
    ),
    responses(
        (status = 200, description = "Device metrics found", body = Vec<DeviceMetric>),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Room not found"),
        (status = 500, description = "Internal Server Error")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "device_metrics"
)]
pub async fn get_metrics_for_room(
    State(router_state): State<Arc<DeviceMetricsRouterState>>,
    Extension(user): Extension<User>,
    Path(room_id): Path<i64>,
    Query(filters): Query<DeviceMetricFilters>,
) -> Result<Json<Vec<DeviceMetric>>> {
    {
        let metrics = router_state
            .device_metrics_service
            .get_metrics_for_room(user.id, room_id, filters)
            .await?;
        Ok(Json(metrics))
    }
}

/// Get device metrics for a house
///
/// Retrieves device metrics for a house with optional filters.
#[utoipa::path(
    get,
    path = "/houses/{house_id}/metrics",
    params(
        ("house_id" = i64, Path, description = "House ID"),
        DeviceMetricFilters
    ),
    responses(
        (status = 200, description = "Device metrics found", body = Vec<DeviceMetric>),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "House not found"),
        (status = 500, description = "Internal Server Error")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "device_metrics"
)]
pub async fn get_metrics_for_house(
    State(router_state): State<Arc<DeviceMetricsRouterState>>,
    Extension(user): Extension<User>,
    Path(house_id): Path<i64>,
    Query(filters): Query<DeviceMetricFilters>,
) -> Result<Json<Vec<DeviceMetric>>> {
    {
        let metrics = router_state
            .device_metrics_service
            .get_metrics_for_house(user.id, house_id, filters)
            .await?;
        Ok(Json(metrics))
    }
}
