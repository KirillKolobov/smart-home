use std::sync::Arc;

use axum::{
    extract::FromRequestParts,
    http::request::Parts,
    routing::{delete, get, post},
    Router,
};

use crate::{
    errors::AppError,
    handlers::houses::{create_house, delete_house, get_user_house_by_id, get_user_houses},
    repositories::{user_houses_repository::UserHousesRepository, HouseRepository},
    routes::rooms::HouseAccess,
    services::{
        access_control_service::{AccessControlService, AccessControlServiceTrait},
        house::HouseService,
    },
    AppState,
};

#[derive(Clone)]
pub struct HousesRouterState {
    pub house_service: HouseService,
    pub access_control_service: AccessControlService,
}

impl HousesRouterState {
    pub fn new(app_state: AppState) -> Self {
        let house_repository = Arc::new(HouseRepository::new(app_state.db.pool.clone()));
        let user_house_repository = Arc::new(UserHousesRepository::new(app_state.db.pool.clone()));
        let house_service = HouseService::new(house_repository, user_house_repository.clone());
        let access_control_service = AccessControlService::new(user_house_repository);

        Self {
            house_service,
            access_control_service,
        }
    }
}

impl FromRequestParts<HousesRouterState> for HouseAccess {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &HousesRouterState,
    ) -> Result<Self, Self::Rejection> {
        let user_id = parts
            .extensions
            .get::<i64>()
            .copied()
            .ok_or_else(|| AppError::AuthorizationError("Not authenticated".to_string()))?;

        let house_id = parts
            .extensions
            .get::<i64>()
            .copied()
            .ok_or_else(|| AppError::BadRequest("Invalid house id".to_string()))?;

        state
            .access_control_service
            .validate_house_access(house_id, user_id)
            .await?;

        Ok(HouseAccess { house_id, user_id })
    }
}

pub fn houses_router(app_state: AppState) -> Router {
    let house_router_state = HousesRouterState::new(app_state.clone());

    Router::new()
        .route("/", get(get_user_houses))
        .route("/", post(create_house))
        .route("/{id}", get(get_user_house_by_id))
        .route("/{id}", delete(delete_house))
        .with_state(house_router_state)
        .merge(crate::routes::device_metrics::device_metrics_routes(
            app_state,
        ))
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::services::{
//         access_control_service::MockAccessControlServiceTrait, house::MockHouseServiceTrait,
//     };
//     use axum::http::Request;
//     use mockall::predicate::eq;
//     use std::sync::Arc;

//     #[tokio::test]
//     async fn test_houses_router_state_new() {
//         let mock_house_service = Arc::new(MockHouseServiceTrait::new());
//         let mock_access_control_service = Arc::new(MockAccessControlServiceTrait::new());

//         let state = HousesRouterState::new(
//             mock_house_service.clone(),
//             mock_access_control_service.clone(),
//         );

//         // Assert that services are created (no panics)
//         assert!(Arc::ptr_eq(
//             &(mock_house_service as Arc<dyn HouseServiceTrait>),
//             &(state.house_service as Arc<dyn HouseServiceTrait>)
//         ));
//         assert!(Arc::ptr_eq(
//             &(mock_access_control_service as Arc<dyn AccessControlServiceTrait>),
//             &(state.access_control_service as Arc<dyn AccessControlServiceTrait>)
//         ));
//     }

//     #[tokio::test]
//     async fn test_house_access_from_request_parts_success() {
//         let mut mock_access_control_service = MockAccessControlServiceTrait::new();
//         mock_access_control_service
//             .expect_validate_house_access()
//             .with(eq(1), eq(1))
//             .times(1)
//             .returning(|_, _| Ok(true));

//         let state = HousesRouterState {
//             house_service: Arc::new(MockHouseServiceTrait::new()),
//             access_control_service: Arc::new(mock_access_control_service),
//         };

//         let mut request = Request::builder()
//             .uri("/houses/1")
//             .body(axum::body::Body::empty())
//             .unwrap();
//         request.extensions_mut().insert(1i64); // Insert user_id
//         request.extensions_mut().insert(1i64); // Insert house_id

//         let (mut parts, _) = request.into_parts();

//         let house_access = HouseAccess::from_request_parts(&mut parts, &state)
//             .await
//             .expect("HouseAccess should be extracted successfully");

//         assert_eq!(house_access.house_id, 1);
//         assert_eq!(house_access.user_id, 1);
//     }

//     #[tokio::test]
//     async fn test_house_access_from_request_parts_unauthenticated() {
//         let mock_house_service = Arc::new(MockHouseServiceTrait::new());
//         let mock_access_control_service = Arc::new(MockAccessControlServiceTrait::new());
//         let state = HousesRouterState::new(mock_house_service, mock_access_control_service);

//         let request = Request::builder()
//             .uri("/houses/1")
//             .body(axum::body::Body::empty())
//             .unwrap();

//         let (mut parts, _) = request.into_parts();

//         let result = HouseAccess::from_request_parts(&mut parts, &state).await;

//         assert!(result.is_err());
//         let err = result.unwrap_err();
//         assert_eq!(
//             err.to_string(),
//             AppError::AuthorizationError("Not authenticated".to_string()).to_string()
//         );
//     }

//     #[tokio::test]
//     async fn test_house_access_from_request_parts_access_denied() {
//         let mut mock_access_control_service = MockAccessControlServiceTrait::new();
//         mock_access_control_service
//             .expect_validate_house_access()
//             .with(eq(1), eq(1))
//             .times(1)
//             .returning(|_, _| Err(AppError::AuthenticationError("Access denied".to_string())));

//         let state = HousesRouterState {
//             house_service: Arc::new(MockHouseServiceTrait::new()),
//             access_control_service: Arc::new(mock_access_control_service),
//         };

//         let mut request = Request::builder()
//             .uri("/houses/1")
//             .body(axum::body::Body::empty())
//             .unwrap();
//         request.extensions_mut().insert(1i64); // Insert user_id
//         request.extensions_mut().insert(1i64); // Insert house_id

//         let (mut parts, _) = request.into_parts();

//         let result = HouseAccess::from_request_parts(&mut parts, &state).await;

//         assert!(result.is_err());
//         assert_eq!(
//             result.unwrap_err().to_string(),
//             AppError::AuthenticationError("Access denied".to_string()).to_string()
//         );
//     }
// }
