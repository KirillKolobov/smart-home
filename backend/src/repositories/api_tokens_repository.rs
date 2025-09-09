use crate::{
    errors::Result,
    models::api_tokens::{ApiToken},
};
use async_trait::async_trait;
use mockall::automock;
use sqlx::PgPool;

#[automock]
#[async_trait]
pub trait ApiTokensRepositoryTrait {
    async fn create(&self, user_id: i64, name: &str, token_hash: &str) -> Result<ApiToken>;
    async fn find_by_user_id(&self, user_id: i64) -> Result<Vec<ApiToken>>;
}

#[derive(Clone)]
pub struct ApiTokensRepository {
    pool: PgPool,
}

impl ApiTokensRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ApiTokensRepositoryTrait for ApiTokensRepository {
    async fn create(&self, user_id: i64, name: &str, token_hash: &str) -> Result<ApiToken> {
        let token = sqlx::query_as!(
            ApiToken,
            r#"
            INSERT INTO api_tokens (user_id, name, token_hash)
            VALUES ($1, $2, $3)
            RETURNING id, user_id, name, token_hash, created_at
            "#,
            user_id,
            name,
            token_hash
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(token)
    }

    async fn find_by_user_id(&self, user_id: i64) -> Result<Vec<ApiToken>> {
        let tokens = sqlx::query_as!(
            ApiToken,
            r#"
            SELECT id, user_id, name, token_hash, created_at
            FROM api_tokens
            WHERE user_id = $1
            ORDER BY created_at DESC
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(tokens)
    }
}
