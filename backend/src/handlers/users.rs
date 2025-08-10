use crate::{db::Database, models::users::User};
use axum::{Json, extract::State, http::StatusCode};

#[utoipa::path(
    get,
    path = "/users/{id}",
    responses(
        (status = 200, description = "Success", body = User),
        (status = 404, description = "Not Found", body = String),
        (status = 500, description = "Internal Server Error", body = String)
    ),
    tag = "users"
)]
pub async fn get_user(
    State(db): State<Database>,
    path: axum::extract::Path<i32>,
) -> Result<(StatusCode, Json<User>), (StatusCode, String)> {
    db.get_user(path.0)
        .await
        .map(|user| (StatusCode::OK, Json(user)))
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, "User not found".to_string()),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
        })
}

#[utoipa::path(
    delete,
    path = "/users/{id}",
    responses(
        (status = 204, description = "No Content"),
        (status = 404, description = "Not Found", body = String),
        (status = 500, description = "Internal Server Error", body = String)
    ),
    tag = "users"
)]
pub async fn delete_user(
    State(db): State<Database>,
    path: axum::extract::Path<i32>,
) -> Result<StatusCode, (StatusCode, String)> {
    db.delete_user(path.0)
        .await
        .map(|_| StatusCode::NO_CONTENT)
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, "User not found".to_string()),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
        })
}
