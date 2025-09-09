use crate::{
    errors::Result,
    models::{
        api_tokens::{CreateApiToken, NewApiToken, PublicApiToken},
        users::User,
    },
    // This will be created in the next step
    routes::api_tokens::ApiTokensRouterState,
};
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json},
    Extension,
};

/// Create a new API token
#[axum::debug_handler]
#[utoipa::path(
    post,
    path = "/api/tokens",
    request_body = CreateApiToken,
    responses(
        (status = 201, description = "API token created successfully", body = NewApiToken),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal Server Error")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "tokens"
)]
pub async fn create_api_token(
    State(state): State<ApiTokensRouterState>,
    Extension(user): Extension<User>,
    Json(payload): Json<CreateApiToken>,
) -> Result<impl IntoResponse> {
    let new_token = state
        .tokens_service
        .create_api_token(user.id, payload)
        .await?;
    Ok((StatusCode::CREATED, Json(new_token)))
}

/// Get all API tokens for the current user
#[utoipa::path(
    get,
    path = "/api/tokens",
    responses(
        (status = 200, description = "List of API tokens", body = Vec<PublicApiToken>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal Server Error")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "tokens"
)]
pub async fn get_api_tokens(
    State(state): State<ApiTokensRouterState>,
    Extension(user): Extension<User>,
) -> Result<Json<Vec<PublicApiToken>>> {
    let tokens = state.tokens_service.get_api_tokens(user.id).await?;
    Ok(Json(tokens))
}
