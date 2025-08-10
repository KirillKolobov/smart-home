use utoipa::OpenApi;

use crate::models::{
    auth::{AuthResponse, LoginRequest, RegisterUser},
    users::User,
};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::handlers::auth::login,
        crate::handlers::auth::register,
        crate::handlers::users::get_user,
        crate::handlers::users::delete_user
    ),
    components(
        schemas(LoginRequest, AuthResponse, User, RegisterUser)
    ),
    tags(
        (name = "auth", description = "Authentication endpoints"),
        (name = "users", description = "User management endpoints")
    )
)]
pub struct ApiDoc;

pub fn generate_openapi_json() -> String {
    ApiDoc::openapi().to_pretty_json().unwrap()
}
