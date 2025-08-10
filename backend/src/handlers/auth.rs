use crate::{
    models::{
        auth::{AuthResponse, LoginRequest, RegisterUser},
        users::User,
    },
    routes::auth::AuthRouterState,
};
use axum::{Json, extract::State, http::StatusCode};
use bcrypt::{DEFAULT_COST, hash};
use validator::Validate;

pub async fn login(
    State(state): State<AuthRouterState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, String)> {
    state
        .auth_service
        .login(payload)
        .await
        .map(Json)
        .map_err(|e| (StatusCode::UNAUTHORIZED, e))
}

pub async fn register(
    State(state): State<AuthRouterState>,
    Json(payload): Json<RegisterUser>,
) -> Result<(StatusCode, Json<User>), (StatusCode, String)> {
    payload
        .validate()
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Validation error: {}", e)))?;

    let processed_payload = hash(&payload.password, DEFAULT_COST)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        .map(|password_hash| RegisterUser {
            password: password_hash,
            ..payload
        })?;

    state
        .app_state
        .db
        .create_user(processed_payload)
        .await
        .map(|user| (StatusCode::CREATED, Json(user)))
        .map_err(|e| match e {
            _ => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
        })
}
