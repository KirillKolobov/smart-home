use bcrypt::verify;
use chrono::Utc;
use jsonwebtoken::{EncodingKey, Header, encode};

use crate::{
    config::Config,
    db::Database,
    models::auth::{AuthResponse, Claims, LoginRequest},
};

#[derive(Clone)]
pub struct AuthService {
    config: Config,
    db: Database,
}

impl AuthService {
    pub fn new(config: Config, db: Database) -> Self {
        Self { config, db }
    }

    pub async fn login(&self, login: LoginRequest) -> Result<AuthResponse, String> {
        let user = self
            .db
            .get_password_hash_by_email(&login.email)
            .await
            .map_err(|_| "Invalid credentials".to_string())?;

        if !verify(login.password, &user.password_hash)
            .map_err(|_| "Password verification failed".to_string())?
        {
            return Err("Invalid credentials".to_string());
        }

        self.generate_token(user.id)
    }

    pub fn generate_token(&self, user_id: i64) -> Result<AuthResponse, String> {
        let expiration = Utc::now()
            .checked_add_signed(chrono::Duration::seconds(self.config.jwt_expires_in as i64))
            .expect("Invalid timestamp")
            .timestamp() as usize;

        let claims = Claims {
            sub: user_id,
            exp: expiration,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.config.jwt_secret.as_ref()),
        )
        .map_err(|e| e.to_string())?;

        Ok(AuthResponse { token, user_id })
    }
}
