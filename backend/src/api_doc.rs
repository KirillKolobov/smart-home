use utoipa::OpenApi;

use crate::models::{
    auth::{AuthResponse, LoginRequest, RegisterUser}, devices::{CreateDevice, Device, UpdateDevice}, houses::{House, NewHouse}, rooms::{NewRoom, Room}, users::{User, UserProfile}
};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::handlers::auth::login,
        crate::handlers::auth::register,
        crate::handlers::users::get_user,
        crate::handlers::users::get_user_profile,
        crate::handlers::users::delete_user,
        crate::handlers::houses::get_user_houses,
        crate::handlers::houses::get_user_house_by_id,
        crate::handlers::houses::create_house,
        crate::handlers::houses::delete_house, 
        crate::handlers::rooms::get_house_rooms,
        crate::handlers::rooms::create_room,
        crate::handlers::rooms::delete_room,
        crate::handlers::devices::get_devices_by_house_id,
        crate::handlers::devices::get_devices_by_room_id,
        crate::handlers::devices::get_device_by_id,
        crate::handlers::devices::create_device,
        crate::handlers::devices::update_device,
        crate::handlers::devices::delete_device
    ),
    components(
        schemas(LoginRequest, AuthResponse, User, UserProfile, RegisterUser, NewHouse, House, Room, NewRoom, CreateDevice, Device, UpdateDevice)
    ),
    tags(
        (name = "auth", description = "Authentication endpoints"),
        (name = "users", description = "User management endpoints"),
        (name = "houses", description = "House management endpoints"),
        (name = "rooms", description = "Room management endpoints"),
        (name = "devices", description = "Device management endpoints"),
        (name = "health", description = "Health check endpoints")
    ),
    info(
        title = "Smart Home Backend API",
        version = "0.1.0",
        description = "REST API for Smart Home management system",
        contact(
            name = "Smart Home Team",
            email = "support@smarthome.com"
        ),
        license(
            name = "MIT",
            url = "https://opensource.org/licenses/MIT"
        )
    ),
    servers(
        (url = "/", description = "Local server")
    )
)]
pub struct ApiDoc;

pub fn generate_openapi_json() -> String {
    ApiDoc::openapi().to_pretty_json().unwrap()
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_openapi_generation() {
//         let json = generate_openapi_json();
//         assert!(!json.is_empty());
//         assert!(json.contains("Smart Home Backend API"));
//         assert!(json.contains("/login"));
//         assert!(json.contains("/register"));
//         assert!(json.contains("/users"));
//         assert!(json.contains("/health"));
//     }

//     #[test]
//     fn test_openapi_structure() {
//         let openapi = ApiDoc::openapi();

//         // Verify basic structure
//         assert_eq!(openapi.info.title, "Smart Home Backend API");
//         assert_eq!(openapi.info.version, "0.1.0");

//         let default_tags: Vec<utoipa::openapi::Tag> = vec![];
//         // Verify tags
//         let tag_names: Vec<&str> = openapi
//             .tags
//             .as_ref()
//             .unwrap_or(&default_tags)
//             .iter()
//             .map(|tag| tag.name.as_str())
//             .collect();

//         assert!(tag_names.contains(&"auth"));
//         assert!(tag_names.contains(&"users"));
//         assert!(tag_names.contains(&"health"));
//     }

//     #[test]
//     fn test_openapi_schemas() {
//         let openapi = ApiDoc::openapi();

//         if let Some(components) = &openapi.components {
//             if let Some(schemas) = &components.schemas {
//                 // Check that our main schemas are present
//                 let schema_names: Vec<&str> = schemas.keys().map(|k| k.as_str()).collect();

//                 // We expect these schemas to be generated
//                 assert!(schema_names
//                     .iter()
//                     .any(|name| name.contains("LoginRequest") || name.contains("AuthResponse")));
//                 assert!(schema_names.iter().any(|name| name.contains("User")));
//                 assert!(schema_names
//                     .iter()
//                     .any(|name| name.contains("RegisterUser")));
//             }
//         }
//     }
// }
