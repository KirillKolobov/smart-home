use crate::{
    errors::Result,
    models::api_tokens::{CreateApiToken, NewApiToken, PublicApiToken},
    repositories::api_tokens_repository::ApiTokensRepositoryTrait,
};
use bcrypt::{hash, DEFAULT_COST};
use rand::Rng;
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
        let plaintext_token: String = (0..32)
            .map(|_| {
                let mut rng = rand::rng();
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::api_tokens::ApiToken;
    use crate::repositories::api_tokens_repository::MockApiTokensRepositoryTrait;
    use chrono::Utc;
    use mockall::predicate::eq;

    #[tokio::test]
    async fn test_create_api_token() {
        let mut mock_repo = MockApiTokensRepositoryTrait::new();
        let user_id = 1;
        let token_name = "test_token".to_string();

        let expected_token = ApiToken {
            id: 1,
            user_id,
            name: token_name.clone(),
            token_hash: "hashed_token".to_string(),
            created_at: Utc::now(),
        };

        let token_name_clone = token_name.clone();
        mock_repo
            .expect_create()
            .withf(move |uid, name, _hash| *uid == user_id && name == token_name_clone)
            .times(1)
            .returning(move |_, _, _| Ok(expected_token.clone()));

        let service = ApiTokensService::new(Arc::new(mock_repo));
        let result = service
            .create_api_token(user_id, CreateApiToken { name: token_name })
            .await;

        assert!(result.is_ok());
        let new_token = result.unwrap();
        assert_eq!(new_token.name, "test_token");
        assert!(!new_token.token.is_empty());
    }

    #[tokio::test]
    async fn test_get_api_tokens() {
        let mut mock_repo = MockApiTokensRepositoryTrait::new();
        let user_id = 1;

        let tokens = vec![
            ApiToken {
                id: 1,
                user_id,
                name: "token1".to_string(),
                token_hash: "hash1".to_string(),
                created_at: Utc::now(),
            },
            ApiToken {
                id: 2,
                user_id,
                name: "token2".to_string(),
                token_hash: "hash2".to_string(),
                created_at: Utc::now(),
            },
        ];

        mock_repo
            .expect_find_by_user_id()
            .with(eq(user_id))
            .times(1)
            .returning(move |_| Ok(tokens.clone()));

        let service = ApiTokensService::new(Arc::new(mock_repo));
        let result = service.get_api_tokens(user_id).await;

        assert!(result.is_ok());
        let public_tokens = result.unwrap();
        assert_eq!(public_tokens.len(), 2);
        assert_eq!(public_tokens[0].name, "token1");
        assert_eq!(public_tokens[1].name, "token2");
    }
}
