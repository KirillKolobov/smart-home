use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Database, Decode, FromRow, Postgres};
use utoipa::ToSchema;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum UserRole {
    User,
    Admin,
}

impl From<String> for UserRole {
    fn from(value: String) -> Self {
        match value.as_str() {
            "admin" => UserRole::Admin,
            _ => UserRole::User,
        }
    }
}

impl ToString for UserRole {
    fn to_string(&self) -> String {
        match self {
            UserRole::User => "user".to_string(),
            UserRole::Admin => "admin".to_string(),
        }
    }
}

impl<'r> Decode<'r, Postgres> for UserRole {
    fn decode(
        value: <Postgres as Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <&str as Decode<Postgres>>::decode(value)?;
        Ok(match s {
            "admin" => UserRole::Admin,
            "user" => UserRole::User,
            unknown => {
                tracing::warn!("Unknown role in DB: '{}', defaulting to 'user'", unknown);
                UserRole::User
            }
        })
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema, Clone)]
pub struct User {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub phone: String,
    pub email: String,
}

#[derive(Debug, Serialize, FromRow, Clone)]
pub struct UserEntity {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub phone: String,
    pub email: String,
    pub role: UserRole,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_login_at: Option<DateTime<Utc>>,
}

impl From<UserEntity> for User {
    fn from(entity: UserEntity) -> Self {
        Self {
            id: entity.id,
            first_name: entity.first_name,
            last_name: entity.last_name,
            phone: entity.phone,
            email: entity.email,
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UserProfile {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub phone: String,
    pub email: String,
    pub role: UserRole,
    pub created_at: DateTime<Utc>,
    pub last_login_at: Option<DateTime<Utc>>,
}

impl From<UserEntity> for UserProfile {
    fn from(entity: UserEntity) -> Self {
        Self {
            id: entity.id,
            first_name: entity.first_name,
            last_name: entity.last_name,
            phone: entity.phone,
            email: entity.email,
            role: entity.role,
            created_at: entity.created_at,
            last_login_at: entity.last_login_at,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_user_entity_to_user_conversion() {
        let entity = UserEntity {
            id: 1,
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            phone: "1234567890".to_string(),
            email: "test@example.com".to_string(),
            role: UserRole::User,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            last_login_at: None,
        };

        let user: User = entity.clone().into();

        assert_eq!(user.id, entity.id);
        assert_eq!(user.first_name, entity.first_name);
        assert_eq!(user.last_name, entity.last_name);
        assert_eq!(user.phone, entity.phone);
        assert_eq!(user.email, entity.email);
    }

    #[test]
    fn test_user_entity_to_profile_conversion() {
        let now = Utc::now();
        let entity = UserEntity {
            id: 1,
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            phone: "1234567890".to_string(),
            email: "test@example.com".to_string(),
            role: UserRole::Admin,
            created_at: now,
            updated_at: now,
            last_login_at: Some(now),
        };

        let profile: UserProfile = entity.clone().into();

        assert_eq!(profile.id, entity.id);
        assert_eq!(profile.first_name, entity.first_name);
        assert_eq!(profile.last_name, entity.last_name);
        assert_eq!(profile.phone, entity.phone);
        assert_eq!(profile.email, entity.email);
        assert_eq!(profile.role, entity.role);
        assert_eq!(profile.created_at, entity.created_at);
        assert_eq!(profile.last_login_at, entity.last_login_at);
    }
}
