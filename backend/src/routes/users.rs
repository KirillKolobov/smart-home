use crate::{
    AppState,
    handlers::users::{delete_user, get_user},
};
use axum::{
    Router,
    routing::{delete, get},
};

pub fn users_router(app_state: AppState) -> Router {
    Router::new()
        .route("/{id}", get(get_user))
        .route("/{id}", delete(delete_user))
        .with_state(app_state.db)
}
