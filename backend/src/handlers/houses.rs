use axum::{
    extract::{Path, State},
    Extension, Json,
};

use crate::{
    errors::Result,
    models::houses::{House, NewHouse},
    routes::houses::HousesRouterState,
    services::house::HouseServiceTrait,
};

/// Get user houses endpoint
///
/// Retrieves a list of houses associated with the authenticated user.
#[utoipa::path(
    get,
    path = "/houses",
    responses(
        (status = 200, description = "Houses found", body = Vec<House>),
        (status = 500, description = "Internal Server Error", body = String)
    ),
    tag = "houses"
)]
pub async fn get_user_houses(
    State(state): State<HousesRouterState>,
    Extension(user_id): Extension<i64>,
) -> Result<Json<Vec<House>>> {
    let houses = state.house_service.get_user_houses(user_id).await?;

    Ok(Json(houses))
}

/// Get house by ID endpoint
///
/// Retrieves a specific house by its ID.
#[utoipa::path(
    get,
    path = "/houses/{id}",
    params(
        ("id" = i64, Path, description = "House ID")
    ),
    responses(
        (status = 200, description = "House found", body = House),
        (status = 404, description = "House not found", body = String),
        (status = 500, description = "Internal Server Error", body = String)
    ),
    tag = "houses"
)]
pub async fn get_user_house_by_id(
    State(state): State<HousesRouterState>,
    Path(house_id): Path<i64>,
) -> Result<Json<House>> {
    let user = state.house_service.get_house_by_id(house_id).await?;

    Ok(Json(user))
}

/// Create house endpoint
///
/// Creates a new house for the authenticated user.
#[utoipa::path(
    post,
    path = "/houses",
    request_body = NewHouse,
    responses(
        (status = 201, description = "House created successfully", body = House),
        (status = 400, description = "Bad Request - Invalid input", body = String),
        (status = 500, description = "Internal Server Error", body = String)
    ),
    tag = "houses"
)]
pub async fn create_house(
    State(state): State<HousesRouterState>,
    Extension(user_id): Extension<i64>,
    Json(payload): Json<NewHouse>,
) -> Result<Json<House>> {
    let house = state.house_service.create_house(user_id, payload).await?;

    Ok(Json(house))
}

/// Delete house endpoint
///
/// Deletes a house by its ID.
#[utoipa::path(
    delete,
    path = "/houses/{id}",
    params(
        ("id" = i64, Path, description = "House ID")
    ),
    responses(
        (status = 200, description = "House deleted successfully", body = ()),
        (status = 404, description = "House not found", body = String),
        (status = 500, description = "Internal Server Error", body = String)
    ),
    tag = "houses"
)]
pub async fn delete_house(
    State(state): State<HousesRouterState>,
    Path(house_id): Path<i64>,
) -> Result<Json<()>> {
    state.house_service.delete_house(house_id).await?;

    Ok(Json(()))
}
