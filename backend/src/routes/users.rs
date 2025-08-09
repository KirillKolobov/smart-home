use crate::{
    db::Database,
    handlers::users::{create_user, get_user, delete_user},
};
use axum::{routing::{delete, get, post}, Router};

pub fn users_router(db: Database) -> Router {
    Router::new()
        .route("/", post(create_user))
        .route("/{id}", get(get_user))
        .route("/{id}", delete(delete_user))
        .with_state(db)
}
