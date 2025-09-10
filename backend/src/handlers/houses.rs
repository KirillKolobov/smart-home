use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension, Json,
};

use crate::{
    errors::{Result, ValidationErrorResponse},
    middlewares::validator::ValidatedJson,
    models::houses::{House, NewHouse},
    routes::{houses::HousesRouterState, rooms::HouseAccess},
    services::house::HouseServiceTrait,
};

use crate::models::common::ListResponse;

/// Get user houses endpoint
///
/// Retrieves a list of houses associated with the authenticated user.
#[utoipa::path(
    get,
    path = "/houses",
    responses(
        (status = 200, description = "Houses found", body = ListResponse<House>),
        (status = 401, description = "Unauthorized", body = String),
        (status = 500, description = "Internal Server Error", body = String)
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "houses"
)]
pub async fn get_user_houses(
    State(state): State<HousesRouterState>,
    Extension(user_id): Extension<i64>,
) -> Result<Json<ListResponse<House>>> {
    let houses = state.house_service.get_user_houses(user_id).await?;

    Ok(Json(ListResponse { items: houses }))
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
        (status = 401, description = "Unauthorized", body = String),
        (status = 404, description = "House not found", body = String),
        (status = 500, description = "Internal Server Error", body = String)
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "houses"
)]
pub async fn get_user_house_by_id(
    State(state): State<HousesRouterState>,
    Path(house_id): Path<i64>,
) -> Result<Json<House>> {
    let house = state.house_service.get_house_by_id(house_id).await?;

    Ok(Json(house))
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
        (status = 400, description = "Bad Request - Invalid input", body = ValidationErrorResponse),
        (status = 401, description = "Unauthorized", body = String),
        (status = 500, description = "Internal Server Error", body = String)
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "houses"
)]
pub async fn create_house(
    State(state): State<HousesRouterState>,
    Extension(user_id): Extension<i64>,
    ValidatedJson(payload): ValidatedJson<NewHouse>,
) -> Result<(StatusCode, Json<House>)> {
    let house = state.house_service.create_house(user_id, payload).await?;

    Ok((StatusCode::CREATED, Json(house)))
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
        (status = 204, description = "House deleted successfully", body = ()),
        (status = 401, description = "Unauthorized", body = String),
        (status = 404, description = "House not found", body = String),
        (status = 500, description = "Internal Server Error", body = String)
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "houses"
)]
pub async fn delete_house(
    State(state): State<HousesRouterState>,
    HouseAccess { house_id, .. }: HouseAccess,
) -> Result<StatusCode> {
    state.house_service.delete_house(house_id).await?;

    Ok(StatusCode::NO_CONTENT)
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;
    use crate::{
        errors::AppError,
        models::user_houses::UserHouse,
        repositories::{
            house_repository::MockHouseRepositoryTrait,
            user_houses_repository::MockUserHousesRepositoryTrait,
        },
        services::{access_control_service::AccessControlService, house::HouseService},
    };
    use axum::extract::State;
    use chrono::Utc;
    use mockall::predicate::eq;

    #[tokio::test]
    async fn test_get_user_houses_success() {
        let mut mock_house_repo = MockHouseRepositoryTrait::new();
        let mock_user_house_repo = MockUserHousesRepositoryTrait::new();

        let houses = vec![House {
            id: 1,
            name: "Test House".to_string(),
            address: "123 Main St, Anytown, USA".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }];
        let user_houses = houses.clone();

        mock_house_repo
            .expect_get_user_houses()
            .with(eq(1))
            .times(1)
            .returning(move |_| Ok(user_houses.clone()));

        let house_service =
            HouseService::new(Arc::new(mock_house_repo), Arc::new(mock_user_house_repo));
        let access_control_service =
            AccessControlService::new(Arc::new(MockUserHousesRepositoryTrait::new()));
        let state = HousesRouterState {
            house_service,
            access_control_service,
        };

        let result = get_user_houses(State(state), Extension(1)).await;

        assert!(result.is_ok());
        let Json(result_houses) = result.unwrap();

        assert_eq!(result_houses.items.len(), houses.len());
    }

    #[tokio::test]
    async fn test_get_user_house_by_id_success() {
        let mut mock_house_repo = MockHouseRepositoryTrait::new();
        let mock_user_house_repo = MockUserHousesRepositoryTrait::new();

        let house = House {
            id: 1,
            name: "Test House".to_string(),
            address: "123 Main St, Anytown, USA".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let cloned_house = house.clone();

        mock_house_repo
            .expect_get_house_by_id()
            .with(eq(1))
            .times(1)
            .returning(move |_| Ok(house.clone()));

        let house_service =
            HouseService::new(Arc::new(mock_house_repo), Arc::new(mock_user_house_repo));
        let access_control_service =
            AccessControlService::new(Arc::new(MockUserHousesRepositoryTrait::new()));
        let state = HousesRouterState {
            house_service,
            access_control_service,
        };

        let result = get_user_house_by_id(State(state), Path(1)).await;

        assert!(result.is_ok());
        let Json(result_house) = result.unwrap();
        assert_eq!(result_house.id, cloned_house.id);
        assert_eq!(result_house.name, cloned_house.name);
    }

    #[tokio::test]
    async fn test_create_house_success() {
        let mut mock_house_repo = MockHouseRepositoryTrait::new();
        let mut mock_user_house_repo = MockUserHousesRepositoryTrait::new();

        let name = "New Test House".to_string();
        let address = "456 Elm St, Anytown, USA".to_string();

        let new_house = NewHouse {
            name: name.clone(),
            address: address.clone(),
        };

        let created_house = House {
            id: 42,
            name,
            address: address.clone(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let user_house = UserHouse {
            user_id: 1,
            house_id: created_house.id,
        };

        let cloned_created_house = created_house.clone();

        mock_house_repo
            .expect_find_house_by_address()
            .with(eq(address))
            .times(1)
            .returning(|_| Ok(None));

        mock_user_house_repo
            .expect_add_house_to_user()
            .with(eq(user_house.user_id), eq(user_house.house_id))
            .times(1)
            .returning(move |_, _| Ok(user_house.clone()));

        mock_house_repo
            .expect_create_house()
            .with(eq(new_house.clone()))
            .times(1)
            .returning(move |_| Ok(created_house.clone()));

        let house_service =
            HouseService::new(Arc::new(mock_house_repo), Arc::new(mock_user_house_repo));
        let access_control_service =
            AccessControlService::new(Arc::new(MockUserHousesRepositoryTrait::new()));
        let state = HousesRouterState {
            house_service,
            access_control_service,
        };

        let result = create_house(State(state), Extension(1), ValidatedJson(new_house)).await;

        assert!(result.is_ok());
        let (status, Json(result_house)) = result.unwrap();
        assert_eq!(status, StatusCode::CREATED);
        assert_eq!(result_house.id, cloned_created_house.id);
        assert_eq!(result_house.name, cloned_created_house.name);
    }

    #[tokio::test]
    async fn test_delete_house_success() {
        let mut mock_house_repo = MockHouseRepositoryTrait::new();
        let mock_user_house_repo = MockUserHousesRepositoryTrait::new();

        mock_house_repo
            .expect_delete_house()
            .with(eq(1))
            .times(1)
            .returning(|_| Ok(()));

        let house_service =
            HouseService::new(Arc::new(mock_house_repo), Arc::new(mock_user_house_repo));
        let access_control_service =
            AccessControlService::new(Arc::new(MockUserHousesRepositoryTrait::new()));
        let state = HousesRouterState {
            house_service,
            access_control_service,
        };

        let result = delete_house(
            State(state),
            HouseAccess {
                house_id: 1,
                user_id: 1,
            },
        )
        .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_house_not_found() {
        let mut mock_house_repo = MockHouseRepositoryTrait::new();
        let mock_user_house_repo = MockUserHousesRepositoryTrait::new();

        mock_house_repo
            .expect_delete_house()
            .with(eq(1))
            .times(1)
            .returning(|_| Err(AppError::NotFound("House not found".to_string())));

        let house_service =
            HouseService::new(Arc::new(mock_house_repo), Arc::new(mock_user_house_repo));
        let access_control_service =
            AccessControlService::new(Arc::new(MockUserHousesRepositoryTrait::new()));
        let state = HousesRouterState {
            house_service,
            access_control_service,
        };

        let result = delete_house(
            State(state),
            HouseAccess {
                house_id: 1,
                user_id: 1,
            },
        )
        .await;

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            AppError::NotFound("House not found".to_string()).to_string()
        );
    }
}
