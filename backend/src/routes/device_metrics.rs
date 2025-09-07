use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    handlers::device_metrics::{
        create_metric, get_metrics, get_metrics_for_house, get_metrics_for_room,
    },
    repositories::{
        device_metrics_repository::DeviceMetricsRepository,
        user_houses_repository::UserHousesRepository,
    },
    services::{
        access_control_service::{AccessControlService, AccessControlServiceTrait},
        device_metrics::{DeviceMetricsService, DeviceMetricsServiceTrait},
    },
    AppState,
};

#[derive(Clone)]
pub struct DeviceMetricsRouterState {
    pub device_metrics_service: Arc<dyn DeviceMetricsServiceTrait + Send + Sync>,
    pub access_control_service: Arc<dyn AccessControlServiceTrait + Send + Sync>,
}

impl DeviceMetricsRouterState {
    pub fn new(app_state: AppState) -> Self {
        let device_metrics_repository =
            Arc::new(DeviceMetricsRepository::new(app_state.db.pool.clone()));
        let user_houses_repo = Arc::new(UserHousesRepository::new(app_state.db.pool.clone()));
        let access_control_service = Arc::new(AccessControlService::new(user_houses_repo));
        let device_metrics_service = Arc::new(DeviceMetricsService::new(
            device_metrics_repository,
            access_control_service.clone(),
        ));

        Self {
            device_metrics_service,
            access_control_service,
        }
    }
}

pub fn device_metrics_router(app_state: AppState) -> Router {
    let device_metrics_router_state = DeviceMetricsRouterState::new(app_state);

    Router::new()
        .route("/", post(create_metric))
        .route("/", get(get_metrics))
        .with_state(Arc::new(device_metrics_router_state))
}

pub fn device_metrics_routes(app_state: AppState) -> Router {
    let device_metrics_router_state = Arc::new(DeviceMetricsRouterState::new(app_state));
    Router::new()
        .route("/houses/{house_id}/metrics", get(get_metrics_for_house))
        .route("/rooms/{room_id}/metrics", get(get_metrics_for_room))
        .with_state(device_metrics_router_state)
}
