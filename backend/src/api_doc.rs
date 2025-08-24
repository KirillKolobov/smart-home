use utoipa::{Modify, OpenApi};

use crate::{handlers, models};

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.get_or_insert_with(Default::default);
        components.add_security_scheme(
            "bearer_auth",
            utoipa::openapi::security::SecurityScheme::Http(
                utoipa::openapi::security::HttpBuilder::new()
                    .scheme(utoipa::openapi::security::HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        );
    }
}

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::auth::login,
        handlers::auth::register,
        handlers::users::get_user_profile,
        handlers::houses::get_user_houses,
        handlers::houses::get_user_house_by_id,
        handlers::houses::create_house,
        handlers::houses::delete_house,
        handlers::rooms::get_house_rooms,
        handlers::rooms::create_room,
        handlers::rooms::delete_room,
        handlers::devices::get_devices_by_house_id,
        handlers::devices::get_devices_by_room_id,
        handlers::devices::get_device_by_id,
        handlers::devices::create_device,
        handlers::devices::update_device,
        handlers::devices::delete_device,
        crate::health_check
    ),
    components(
        schemas(
            models::auth::LoginRequest,
            models::auth::AuthResponse,
            models::users::User,
            models::auth::RegisterUser,
            models::houses::NewHouse,
            models::houses::House,
            models::rooms::Room,
            models::rooms::NewRoom,
            models::devices::CreateDevice,
            models::devices::Device,
            models::devices::UpdateDevice
        )
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
    ),
    modifiers(&SecurityAddon)
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
//         assert!(json.contains("/auth/login"));
//         assert!(json.contains("/auth/signup"));
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
