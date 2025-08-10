use crate::{
    AppState,
    handlers::auth::{login, register},
    services::auth::AuthService,
};
use axum::{Router, routing::post};

#[derive(Clone)]
pub struct AuthRouterState {
    pub auth_service: AuthService,
    pub app_state: AppState,
}

pub fn auth_router(app_state: AppState) -> Router {
    let auth_router_state = AuthRouterState {
        auth_service: AuthService::new(app_state.config.clone(), app_state.db.clone()),
        app_state: app_state.clone(),
    };

    Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
        .with_state(auth_router_state)
}
