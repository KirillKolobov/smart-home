use axum::{
    extract::{Path, State},
    Json,
};

use crate::{
    errors::Result,
    models::rooms::{NewRoom, Room},
    routes::rooms::{HouseAccess, RoomsRouterState},
    services::rooms::RoomsServiceTrait,
};

/// Get house rooms endpoint
///
/// Retrieves a list of rooms associated with a specific house.
#[utoipa::path(
    get,
    path = "/houses/{id}/rooms",
    responses(
        (status = 200, description = "Rooms found", body = Vec<Room>),
        (status = 500, description = "Internal Server Error", body = String)
    ),
    tag = "rooms"
)]
#[axum::debug_handler]
pub async fn get_house_rooms(
    State(state): State<RoomsRouterState>,
    HouseAccess {
        house_id,
        user_id: _,
    }: HouseAccess,
) -> Result<Json<Vec<Room>>> {
    let rooms = state.room_service.get_house_rooms(house_id).await?;

    Ok(Json(rooms))
}

/// Create new room for house
///
/// Creates a new room for a specific house.
#[utoipa::path(
    get,
    path = "/houses/{id}/rooms",
    responses(
        (status = 201, description = "Room created", body = Room),
        (status = 500, description = "Internal Server Error", body = String)
    ),
    tag = "rooms"
)]
#[axum::debug_handler]
pub async fn create_room(
    State(state): State<RoomsRouterState>,
    house_access: HouseAccess,
    Json(payload): Json<NewRoom>,
) -> Result<Json<Room>> {
    let rooms = state
        .room_service
        .create_house_room(house_access.house_id, payload)
        .await?;

    Ok(Json(rooms))
}

/// Delete room from house
///
/// Deletes a room from a specific house.
#[utoipa::path(
    get,
    path = "/houses/{id}/rooms/{id}",
    responses(
        (status = 200, description = "Room deleted", body = ()),
        (status = 500, description = "Internal Server Error", body = String)
    ),
    tag = "rooms"
)]
#[axum::debug_handler]
pub async fn delete_room(
    State(state): State<RoomsRouterState>,
    HouseAccess {
        house_id: _,
        user_id: _,
    }: HouseAccess,
    Path(room_id): Path<i64>,
) -> Result<Json<()>> {
    state.room_service.delete_room(room_id).await?;

    Ok(Json(()))
}
