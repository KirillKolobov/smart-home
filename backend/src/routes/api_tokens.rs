use crate::{
    handlers::api_tokens::{create_api_token, get_api_tokens},
    repositories::ApiTokensRepository,
    services::api_tokens::ApiTokensService,
    AppState,
};
use axum::{routing::post, Router};
use std::sync::Arc;

#[derive(Clone)]
pub struct ApiTokensRouterState {
    pub tokens_service: ApiTokensService,
}

impl ApiTokensRouterState {
    pub fn new(app_state: AppState) -> Self {
        let tokens_repository = Arc::new(ApiTokensRepository::new(app_state.db.pool.clone()));
        let tokens_service = ApiTokensService::new(tokens_repository);

        Self { tokens_service }
    }
}

pub fn api_tokens_router(app_state: AppState) -> Router {
    let router_state: ApiTokensRouterState = ApiTokensRouterState::new(app_state);

    Router::new()
        .route("/", post(create_api_token).get(get_api_tokens))
        .with_state(router_state)
}
