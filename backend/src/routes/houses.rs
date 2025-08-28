use std::sync::Arc;

use axum::{ 
    extract::{FromRequestParts, Path},
    http::request::Parts,
    routing::{delete, get, post},
    Router,
};

use crate::{
    errors::AppError,
    handlers::houses::{create_house, delete_house, get_user_house_by_id, get_user_houses},
    repositories::{user_houses_repository::UserHousesRepository, HouseRepository},
    routes::rooms::HouseAccess,
    services::{
        access_control_service::{AccessControlService, AccessControlServiceTrait},
        house::HouseService,
    },
    AppState,
};

#[derive(Clone)]
pub struct HousesRouterState {
    pub house_service: HouseService,
    pub access_control_service: AccessControlService,
}

impl HousesRouterState {
    pub fn new(app_state: AppState) -> Self {
        let house_repository = Arc::new(HouseRepository::new(app_state.db.pool.clone()));
        let user_house_repository = Arc::new(UserHousesRepository::new(app_state.db.pool.clone()));
        let house_service = HouseService::new(house_repository, user_house_repository.clone());
        let access_control_service = AccessControlService::new(user_house_repository);

        Self {
            house_service,
            access_control_service,
        }
    }
}

impl FromRequestParts<HousesRouterState> for HouseAccess {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &HousesRouterState,
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

pub fn houses_router(app_state: AppState) -> Router {
    let house_router_state = HousesRouterState::new(app_state.clone());

    Router::new()
        .route("/", get(get_user_houses))
        .route("/", post(create_house))
        .route("/{id}", get(get_user_house_by_id))
        .route("/{id}", delete(delete_house))
        .with_state(house_router_state)
        .merge(crate::routes::device_metrics::device_metrics_routes(
            app_state,
        ))
}
