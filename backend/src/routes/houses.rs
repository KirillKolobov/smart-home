use std::sync::Arc;

use axum::{
    routing::{delete, get, post},
    Router,
};

use crate::{
    handlers::houses::{create_house, delete_house, get_user_house_by_id, get_user_houses},
    repositories::{user_houses_repository::UserHousesRepository, HouseRepository},
    services::house::HouseService,
    AppState,
};

#[derive(Clone)]
pub struct HousesRouterState {
    pub house_service: HouseService,
}

impl HousesRouterState {
    pub fn new(app_state: AppState) -> Self {
        let house_repository = Arc::new(HouseRepository::new(app_state.db.pool.clone()));
        let user_house_repository = Arc::new(UserHousesRepository::new(app_state.db.pool.clone()));
        let house_service = HouseService::new(house_repository, user_house_repository);

        Self { house_service }
    }
}

pub fn houses_router(app_state: AppState) -> Router {
    let house_router_state = HousesRouterState::new(app_state);

    Router::new()
        .route("/", get(get_user_houses))
        .route("/", post(create_house))
        .route("/{id}", get(get_user_house_by_id))
        .route("/{id}", delete(delete_house))
        .with_state(house_router_state)
}
