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

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::PgPool;

    // Helper function to create a test pool (in-memory or real, depending on setup)
    async fn create_test_pool() -> PgPool {
        let pool = PgPool::connect("postgres://smart_home_user:1234@127.0.0.1:5432/smart_home_test")
            .await
            .expect("Failed to connect to test database");

        // Truncate the api_tokens table to ensure a clean state for each test
        sqlx::query!("TRUNCATE TABLE api_tokens RESTART IDENTITY CASCADE")
            .execute(&pool)
            .await
            .expect("Failed to truncate api_tokens table");

        pool
    }

    #[sqlx::test]
    async fn test_create_api_token() {
        let pool = create_test_pool().await;
        let repo = ApiTokensRepository::new(pool);

        let user_id = 1;
        let name = "test_token".to_string();
        let token_hash = "hashed_token".to_string();

        let created_token = repo
            .create(user_id, &name, &token_hash)
            .await
            .expect("Failed to create API token");

        assert_eq!(created_token.user_id, user_id);
        assert_eq!(created_token.name, name);
        assert_eq!(created_token.token_hash, token_hash);
        assert!(!created_token.id.to_string().is_empty());
    }

    #[sqlx::test]
    async fn test_find_by_user_id() {
        let pool = create_test_pool().await;
        let repo = ApiTokensRepository::new(pool);

        let user_id = 2; // Use a different user ID for this test
        let name1 = "token1".to_string();
        let token_hash1 = "hash1".to_string();
        let name2 = "token2".to_string();
        let token_hash2 = "hash2".to_string();

        // Create some tokens for the user
        repo.create(user_id, &name1, &token_hash1)
            .await
            .expect("Failed to create token1");
        repo.create(user_id, &name2, &token_hash2)
            .await
            .expect("Failed to create token2");

        let tokens = repo
            .find_by_user_id(user_id)
            .await
            .expect("Failed to find tokens by user ID");

        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].name, name2); // Ordered by created_at DESC
        assert_eq!(tokens[1].name, name1);
    }
}
