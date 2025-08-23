use std::sync::Arc;

use axum::{
    extract::{FromRequestParts, Path},
    http::request::Parts,
    routing::{delete, get, patch, post},
    Router,
};

use crate::{
    errors::AppError,
    handlers::devices::{
        create_device, delete_device, get_device_by_id, get_devices_by_house_id,
        get_devices_by_room_id, update_device,
    },
    repositories::user_houses_repository::UserHousesRepository,
    routes::rooms::HouseAccess,
    services::{
        access_control_service::{AccessControlService, AccessControlServiceTrait},
        device::DeviceServiceTrait,
    },
    AppState,
};

#[derive(Clone)]
pub struct DeviceRouterState {
    pub device_service: Arc<dyn DeviceServiceTrait + Send + Sync>,
    pub app_state: AppState,
    pub access_control_service: AccessControlService,
}

impl DeviceRouterState {
    pub fn new(app_state: AppState) -> Self {
        let device_repository = Arc::new(crate::repositories::DeviceRepository::new(
            app_state.db.pool.clone(),
        ));
        let device_service = Arc::new(crate::services::device::DeviceService::new(
            device_repository,
        ));
        let user_houses_repo = Arc::new(UserHousesRepository::new(app_state.db.pool.clone()));
        let access_control_service = AccessControlService::new(user_houses_repo);

        Self {
            device_service,
            app_state,
            access_control_service,
        }
    }
}

impl FromRequestParts<Arc<DeviceRouterState>> for HouseAccess {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<DeviceRouterState>,
    ) -> Result<Self, Self::Rejection> {
        let user_id = parts
            .extensions
            .get::<i64>()
            .copied()
            .ok_or_else(|| AppError::AuthorizationError("Not authenticated".to_string()))?;

        let Path(house_id) = Path::<i64>::from_request_parts(parts, state)
            .await
            .map_err(|_| AppError::BadRequest("Invalid house id".to_string()))?;

        state
            .access_control_service
            .validate_house_access(house_id, user_id)
            .await?;

        Ok(HouseAccess { house_id, user_id })
    }
}

pub fn devices_router(app_state: AppState) -> Router {
    let device_router_state = DeviceRouterState::new(app_state);

    Router::new()
        .route("/", post(create_device))
        .route("/{device_id}", get(get_device_by_id))
        .route("/{device_id}", patch(update_device))
        .route("/{device_id}", delete(delete_device))
        .with_state(Arc::new(device_router_state))
}

pub fn house_devices_router(app_state: AppState) -> Router {
    let device_router_state = DeviceRouterState::new(app_state);

    Router::new()
        .route("/", get(get_devices_by_house_id))
        .with_state(Arc::new(device_router_state))
}

pub fn room_devices_router(app_state: AppState) -> Router {
    let device_router_state = DeviceRouterState::new(app_state);

    Router::new()
        .route("/", get(get_devices_by_room_id))
        .with_state(Arc::new(device_router_state))
}
