use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use validator::Validate;

/// Represents an API token in the database.
#[derive(Debug, FromRow, Serialize, Deserialize, ToSchema, Clone)]
pub struct ApiToken {
    pub id: i64,
    pub user_id: i64,
    pub name: String,
    #[serde(skip_serializing)] // Never expose the hash
    pub token_hash: String,
    pub created_at: DateTime<Utc>,
}

/// Publicly safe information about an API token.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PublicApiToken {
    pub id: i64,
    pub name: String,
    pub created_at: DateTime<Utc>,
}

/// The response when creating a new API token, including the plaintext token.
/// The plaintext token is only shown once.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct NewApiToken {
    pub id: i64,
    pub name: String,
    pub created_at: DateTime<Utc>,
    /// The plaintext token. This is only provided on creation.
    pub token: String,
}

/// The request body for creating a new API token.
#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct CreateApiToken {
    #[validate(length(min = 2, message = "Name must be at least 2 characters"))]
    #[serde(default)]
    pub name: String,
}

impl From<ApiToken> for PublicApiToken {
    fn from(token: ApiToken) -> Self {
        Self {
            id: token.id,
            name: token.name,
            created_at: token.created_at,
        }
    }
}
