use axum::{
    extract::{Path, State},
    Json,
};

use crate::{
    errors::Result,
    models::rooms::{NewRoom, Room},
    routes::rooms::{HouseAccess, RoomsRouterState},
    services::{house::HouseServiceTrait, rooms::RoomsServiceTrait},
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
pub async fn get_house_rooms<R, H, A>(
    State(state): State<RoomsRouterState<R, H, A>>,
    HouseAccess {
        house_id,
        user_id: _,
    }: HouseAccess,
) -> Result<Json<Vec<Room>>>
where
    R: RoomsServiceTrait,
    H: HouseServiceTrait,
    A: crate::services::access_control_service::AccessControlServiceTrait,
{
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
pub async fn create_room<R, H, A>(
    State(state): State<RoomsRouterState<R, H, A>>,
    house_access: HouseAccess,
    Json(payload): Json<NewRoom>,
) -> Result<Json<Room>>
where
    R: RoomsServiceTrait,
    H: HouseServiceTrait,
    A: crate::services::access_control_service::AccessControlServiceTrait,
{
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
pub async fn delete_room<R, H, A>(
    State(state): State<RoomsRouterState<R, H, A>>,
    HouseAccess {
        house_id: _,
        user_id: _,
    }: HouseAccess,
    Path(room_id): Path<i64>,
) -> Result<Json<()>>
where
    R: RoomsServiceTrait,
    H: HouseServiceTrait,
    A: crate::services::access_control_service::AccessControlServiceTrait,
{
    state.room_service.delete_room(room_id).await?;

    Ok(Json(()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        errors::AppError,
        models::rooms::{NewRoom, Room},
        routes::rooms::RoomsRouterState,
        services::{
            access_control_service::MockAccessControlServiceTrait, house::MockHouseServiceTrait,
            rooms::MockRoomsServiceTrait,
        },
    };
    use axum::extract::{Path, State};
    use chrono::Utc;
    use mockall::predicate::*;

    #[tokio::test]
    async fn test_get_house_rooms_success() {
        let mut mock_room_service = MockRoomsServiceTrait::new();
        let mock_house_service = MockHouseServiceTrait::new();
        let mock_access_control_service = MockAccessControlServiceTrait::new();

        let now = Utc::now();
        let room = Room {
            id: 1,
            house_id: 1,
            name: "Living Room".to_string(),
            room_type: "living_room".to_string(),
            created_at: now,
            updated_at: now,
        };

        mock_room_service
            .expect_get_house_rooms()
            .with(eq(1i64))
            .times(1)
            .returning(move |_| Ok(vec![room.clone()]));

        let state = RoomsRouterState {
            room_service: mock_room_service,
            house_service: mock_house_service,
            access_control_service: mock_access_control_service,
        };

        let house_access = HouseAccess {
            house_id: 1,
            user_id: 1,
        };

        let result = get_house_rooms(State(state), house_access).await;

        assert!(result.is_ok());
        let Json(rooms) = result.unwrap();
        assert_eq!(rooms.len(), 1);
        assert_eq!(rooms[0].id, 1);
        assert_eq!(rooms[0].name, "Living Room");
    }

    #[tokio::test]
    async fn test_create_room_success() {
        let mut mock_room_service = MockRoomsServiceTrait::new();
        let mock_house_service = MockHouseServiceTrait::new();
        let mock_access_control_service = MockAccessControlServiceTrait::new();

        let now = Utc::now();
        let new_room = NewRoom {
            name: "Bedroom".to_string(),
            room_type: "bedroom".to_string(),
        };

        let room = Room {
            id: 1,
            house_id: 1,
            name: "Bedroom".to_string(),
            room_type: "bedroom".to_string(),
            created_at: now,
            updated_at: now,
        };

        mock_room_service
            .expect_create_house_room()
            .with(eq(1i64), eq(new_room.clone()))
            .times(1)
            .returning(move |_, _| Ok(room.clone()));

        let state = RoomsRouterState {
            room_service: mock_room_service,
            house_service: mock_house_service,
            access_control_service: mock_access_control_service,
        };

        let house_access = HouseAccess {
            house_id: 1,
            user_id: 1,
        };

        let result = create_room(State(state), house_access, Json(new_room)).await;

        assert!(result.is_ok());
        let Json(created_room) = result.unwrap();
        assert_eq!(created_room.id, 1);
        assert_eq!(created_room.name, "Bedroom");
    }

    #[tokio::test]
    async fn test_delete_room_success() {
        let mut mock_room_service = MockRoomsServiceTrait::new();
        let mock_house_service = MockHouseServiceTrait::new();
        let mock_access_control_service = MockAccessControlServiceTrait::new();

        mock_room_service
            .expect_delete_room()
            .with(eq(1i64))
            .times(1)
            .returning(|_| Ok(()));

        let state = RoomsRouterState {
            room_service: mock_room_service,
            house_service: mock_house_service,
            access_control_service: mock_access_control_service,
        };

        let house_access = HouseAccess {
            house_id: 1,
            user_id: 1,
        };

        let result = delete_room(State(state), house_access, Path(1)).await;

        assert!(result.is_ok());
        let Json(()) = result.unwrap();
    }

    #[tokio::test]
    async fn test_delete_room_not_found() {
        let mut mock_room_service = MockRoomsServiceTrait::new();
        let mock_house_service = MockHouseServiceTrait::new();
        let mock_access_control_service = MockAccessControlServiceTrait::new();

        mock_room_service
            .expect_delete_room()
            .with(eq(999i64))
            .times(1)
            .returning(|_| Err(AppError::NotFound("Room not found".to_string())));

        let state = RoomsRouterState {
            room_service: mock_room_service,
            house_service: mock_house_service,
            access_control_service: mock_access_control_service,
        };

        let house_access = HouseAccess {
            house_id: 1,
            user_id: 1,
        };

        let result = delete_room(State(state), house_access, Path(999)).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::NotFound(_) => (),
            _ => panic!("Expected NotFound error"),
        }
    }
}
