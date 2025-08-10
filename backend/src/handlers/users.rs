use crate::{db::Database, models::users::User};
use axum::{Json, extract::State, http::StatusCode};

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
