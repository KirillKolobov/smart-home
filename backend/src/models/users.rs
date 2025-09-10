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

impl std::fmt::Display for UserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            UserRole::User => write!(f, "user"),
            UserRole::Admin => write!(f, "admin"),
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
            _ => UserRole::User,
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
    pub role: UserRole,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_login_at: Option<DateTime<Utc>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_role_from_string() {
        assert_eq!(UserRole::from("admin".to_string()), UserRole::Admin);
        assert_eq!(UserRole::from("user".to_string()), UserRole::User);
        assert_eq!(UserRole::from("anything_else".to_string()), UserRole::User);
    }

    #[test]
    fn test_user_role_display() {
        assert_eq!(format!("{}", UserRole::User), "user");
        assert_eq!(format!("{}", UserRole::Admin), "admin");
    }
}
