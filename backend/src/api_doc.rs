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
        handlers::api_tokens::create_api_token,
        handlers::api_tokens::get_api_tokens,
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
            models::api_tokens::CreateApiToken,
            models::api_tokens::PublicApiToken,
            models::api_tokens::NewApiToken,
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
        (name = "tokens", description = "API token management endpoints"),
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
