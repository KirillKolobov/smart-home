use crate::{
    errors::Result,
    models::api_tokens::{CreateApiToken, NewApiToken, PublicApiToken},
    repositories::api_tokens_repository::ApiTokensRepositoryTrait,
};
use bcrypt::{hash, DEFAULT_COST};
use rand::{Rng, SeedableRng};
use std::sync::Arc;

#[derive(Clone)]
pub struct ApiTokensService {
    repo: Arc<dyn ApiTokensRepositoryTrait + Send + Sync>,
}

impl ApiTokensService {
    pub fn new(repo: Arc<dyn ApiTokensRepositoryTrait + Send + Sync>) -> Self {
        Self { repo }
    }

    pub async fn create_api_token(
        &self,
        user_id: i64,
        token_data: CreateApiToken,
    ) -> Result<NewApiToken> {
        // 1. Generate a new random token
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
        let mut rng = rand::rngs::StdRng::from_os_rng();
        let plaintext_token: String = (0..32)
            .map(|_| {
                let idx = rng.random_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect();

        // 2. Hash the token
        let token_hash = hash(&plaintext_token, DEFAULT_COST)?;

        // 3. Save to the database
        let created_token = self
            .repo
            .create(user_id, &token_data.name, &token_hash)
            .await?;

        // 4. Return the public data + the plaintext token
        Ok(NewApiToken {
            id: created_token.id,
            name: created_token.name,
            created_at: created_token.created_at,
            token: plaintext_token,
        })
    }

    pub async fn get_api_tokens(&self, user_id: i64) -> Result<Vec<PublicApiToken>> {
        let tokens = self.repo.find_by_user_id(user_id).await?;
        let public_tokens = tokens.into_iter().map(PublicApiToken::from).collect();
        Ok(public_tokens)
    }
}
