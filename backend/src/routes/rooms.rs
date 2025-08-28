use std::sync::Arc;

use axum::{ 
    extract::{FromRequestParts, Path},
    http::request::Parts,
    routing::{delete, get, post},
    Router,
};

use crate::{
    errors::AppError,
    handlers::rooms::{create_room, delete_room, get_house_rooms},
    repositories::{
        rooms_repository::RoomsRepository, user_houses_repository::UserHousesRepository,
        HouseRepository,
    },
    services::{
        access_control_service::{self, AccessControlService},
        house::{self, HouseService},
        rooms::{self, RoomsService},
    },
    AppState,
};

#[derive(Clone)]
pub struct RoomsRouterState<R, H, A> {
    pub room_service: R,
    pub house_service: H,
    pub access_control_service: A,
}

impl RoomsRouterState<RoomsService, HouseService, AccessControlService> {
    pub fn new(app_state: AppState) -> Self {
        let house_repository = Arc::new(HouseRepository::new(app_state.db.pool.clone()));
        let user_house_repository = Arc::new(UserHousesRepository::new(app_state.db.pool.clone()));
        let house_service =
            HouseService::new(house_repository.clone(), user_house_repository.clone());
        let access_control_service = AccessControlService::new(user_house_repository.clone());
        let rooms_repository = Arc::new(RoomsRepository::new(app_state.db.pool.clone()));

        Self {
            house_service,
            access_control_service,
            room_service: RoomsService::new(rooms_repository),
        }
    }
}

#[derive(Debug, Clone)]
pub struct HouseAccess {
    pub house_id: i64,
    pub user_id: i64,
}

impl<R, H, A> FromRequestParts<RoomsRouterState<R, H, A>> for HouseAccess
where
    R: rooms::RoomsServiceTrait + Clone + Send + Sync + 'static,
    H: house::HouseServiceTrait + Clone + Send + Sync + 'static,
    A: access_control_service::AccessControlServiceTrait + Clone + Send + Sync + 'static,
{
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &RoomsRouterState<R, H, A>,
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

pub fn rooms_router(app_state: AppState) -> Router {
    let rooms_router_state = RoomsRouterState::new(app_state.clone());

    Router::new()
        .route("/", get(get_house_rooms))
        .route("/", post(create_room))
        .route("/{id}", delete(delete_room))
        .with_state(rooms_router_state)
        .merge(crate::routes::device_metrics::device_metrics_routes(
            app_state,
        ))
}
