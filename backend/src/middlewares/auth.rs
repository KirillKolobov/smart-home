use crate::{errors::AppError, models::auth::Claims, repositories::UserRepositoryTrait, AppState};
use axum::{
    body::Body,
    extract::{Request, State},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use tracing::{info, warn};

/// Authentication middleware that validates JWT tokens
///
/// This middleware:
/// 1. Extracts the Bearer token from the Authorization header
/// 2. Validates the JWT token
/// 3. Verifies the user still exists in the database
/// 4. Adds the user_id to request extensions for use in handlers
pub async fn auth_middleware(
    State(state): State<AppState>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, AppError> {
    // Extract token from Authorization header
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| {
            warn!("Missing Authorization header");
            AppError::AuthenticationError("Missing Authorization header".to_string())
        })?;

    // Check for Bearer prefix and extract token
    let token = auth_header.strip_prefix("Bearer ").ok_or_else(|| {
        warn!("Invalid Authorization header format");
        AppError::AuthenticationError("Invalid Authorization header format".to_string())
    })?;

    if token.is_empty() {
        warn!("Empty token provided");
        return Err(AppError::AuthenticationError("Empty token".to_string()));
    }

    // Decode and validate JWT token
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(state.config.jwt_secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|e| {
        warn!("Token validation failed: {}", e);
        match e.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                AppError::AuthenticationError("Token has expired".to_string())
            }
            jsonwebtoken::errors::ErrorKind::InvalidToken => {
                AppError::AuthenticationError("Invalid token format".to_string())
            }
            jsonwebtoken::errors::ErrorKind::InvalidSignature => {
                AppError::AuthenticationError("Invalid token signature".to_string())
            }
            _ => AppError::AuthenticationError("Token validation failed".to_string()),
        }
    })?;

    let user_id = token_data.claims.sub;

    // Verify that the user still exists in the database
    let user_repository = crate::repositories::UserRepository::new(state.db.pool.clone());

    match user_repository.get_user_by_id(user_id).await {
        Ok(_) => {
            info!("Authenticated user ID: {}", user_id);
            // Add user_id to request extensions for use in handlers
            req.extensions_mut().insert(user_id);
            Ok(next.run(req).await)
        }
        Err(crate::errors::AppError::NotFound(_)) => {
            warn!("Token valid but user not found: {}", user_id);
            Err(AppError::AuthenticationError(
                "User no longer exists".to_string(),
            ))
        }
        Err(e) => {
            warn!("Database error during user verification: {}", e);
            Err(AppError::InternalServerError(
                "User verification failed".to_string(),
            ))
        }
    }
}

/// Extract user ID from request extensions
///
/// This function should be called from handlers that are protected by auth middleware
/// to get the authenticated user's ID.
pub fn extract_user_id(req: &Request<Body>) -> Result<i64, AppError> {
    req.extensions()
        .get::<i64>()
        .copied()
        .ok_or_else(|| AppError::AuthorizationError("User ID not found in request".to_string()))
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::{config::Config, db::Database, models::auth::Claims};
//     use axum::{body::Body, extract::Request, http::Method, middleware::Next};
//     use chrono::Utc;
//     use jsonwebtoken::{encode, EncodingKey, Header};

//     fn create_test_config() -> Config {
//         Config {
//             port: 3000,
//             db_host: "localhost".to_string(),
//             db_name: "test".to_string(),
//             db_port: 5432,
//             db_user: "test".to_string(),
//             db_pass: "test".to_string(),
//             jwt_secret: "test_secret_key_that_is_long_enough".to_string(),
//             jwt_expires_in: 3600,
//         }
//     }

//     fn create_test_token(user_id: i64, secret: &str, expired: bool) -> String {
//         let expiration = if expired {
//             (Utc::now() - chrono::Duration::hours(1)).timestamp() as usize
//         } else {
//             (Utc::now() + chrono::Duration::hours(1)).timestamp() as usize
//         };

//         let claims = Claims {
//             sub: user_id,
//             exp: expiration,
//         };

//         encode(
//             &Header::default(),
//             &claims,
//             &EncodingKey::from_secret(secret.as_ref()),
//         )
//         .unwrap()
//     }

//     #[tokio::test]
//     async fn test_missing_authorization_header() {
//         let config = create_test_config();
//         let pool = sqlx::postgres::PgPoolOptions::new()
//             .max_connections(1)
//             .connect("postgres://test:test@localhost/nonexistent")
//             .await;

//         if let Ok(pool) = pool {
//             let state = AppState::new(Database::new(pool), config);

//             let req = Request::builder()
//                 .method(Method::GET)
//                 .uri("/test")
//                 .body(Body::empty())
//                 .unwrap();

//             let next = Next::new(|| async move {
//                 panic!("Next should not be called");
//             });

//             let result = auth_middleware(State(state), req, next).await;

//             assert!(result.is_err());
//             match result.unwrap_err() {
//                 AppError::AuthenticationError(msg) => {
//                     assert!(msg.contains("Missing Authorization header"));
//                 }
//                 _ => panic!("Expected AuthenticationError"),
//             }
//         }
//     }

//     #[tokio::test]
//     async fn test_invalid_authorization_header_format() {
//         let config = create_test_config();
//         let pool = sqlx::postgres::PgPoolOptions::new()
//             .max_connections(1)
//             .connect("postgres://test:test@localhost/nonexistent")
//             .await;

//         if let Ok(pool) = pool {
//             let state = AppState::new(Database::new(pool), config);

//             let req = Request::builder()
//                 .method(Method::GET)
//                 .uri("/test")
//                 .header("Authorization", "Invalid token")
//                 .body(Body::empty())
//                 .unwrap();

//             let next = Next::new(|| async move {
//                 panic!("Next should not be called");
//             });

//             let result = auth_middleware(State(state), req, next).await;

//             assert!(result.is_err());
//             match result.unwrap_err() {
//                 AppError::AuthenticationError(msg) => {
//                     assert!(msg.contains("Invalid Authorization header format"));
//                 }
//                 _ => panic!("Expected AuthenticationError"),
//             }
//         }
//     }

//     #[tokio::test]
//     async fn test_empty_token() {
//         let config = create_test_config();
//         let pool = sqlx::postgres::PgPoolOptions::new()
//             .max_connections(1)
//             .connect("postgres://test:test@localhost/nonexistent")
//             .await;

//         if let Ok(pool) = pool {
//             let state = AppState::new(Database::new(pool), config);

//             let req = Request::builder()
//                 .method(Method::GET)
//                 .uri("/test")
//                 .header("Authorization", "Bearer ")
//                 .body(Body::empty())
//                 .unwrap();

//             let next = Next::new(|| async move {
//                 panic!("Next should not be called");
//             });

//             let result = auth_middleware(State(state), req, next).await;

//             assert!(result.is_err());
//             match result.unwrap_err() {
//                 AppError::AuthenticationError(msg) => {
//                     assert!(msg.contains("Empty token"));
//                 }
//                 _ => panic!("Expected AuthenticationError"),
//             }
//         }
//     }

//     #[tokio::test]
//     async fn test_invalid_token() {
//         let config = create_test_config();
//         let pool = sqlx::postgres::PgPoolOptions::new()
//             .max_connections(1)
//             .connect("postgres://test:test@localhost/nonexistent")
//             .await;

//         if let Ok(pool) = pool {
//             let state = AppState::new(Database::new(pool), config);

//             let req = Request::builder()
//                 .method(Method::GET)
//                 .uri("/test")
//                 .header("Authorization", "Bearer invalid.token.here")
//                 .body(Body::empty())
//                 .unwrap();

//             let next = Next::new(|| async move {
//                 panic!("Next should not be called");
//             });

//             let result = auth_middleware(State(state), req, next).await;

//             assert!(result.is_err());
//             match result.unwrap_err() {
//                 AppError::AuthenticationError(_) => (),
//                 _ => panic!("Expected AuthenticationError"),
//             }
//         }
//     }

//     #[tokio::test]
//     async fn test_expired_token() {
//         let config = create_test_config();
//         let pool = sqlx::postgres::PgPoolOptions::new()
//             .max_connections(1)
//             .connect("postgres://test:test@localhost/nonexistent")
//             .await;

//         if let Ok(pool) = pool {
//             let state = AppState::new(Database::new(pool), config.clone());

//             let expired_token = create_test_token(1, &config.jwt_secret, true);

//             let req = Request::builder()
//                 .method(Method::GET)
//                 .uri("/test")
//                 .header("Authorization", format!("Bearer {}", expired_token))
//                 .body(Body::empty())
//                 .unwrap();

//             let next = Next::new(|| async move {
//                 panic!("Next should not be called");
//             });

//             let result = auth_middleware(State(state), req, next).await;

//             assert!(result.is_err());
//             match result.unwrap_err() {
//                 AppError::AuthenticationError(msg) => {
//                     assert!(msg.contains("expired"));
//                 }
//                 _ => panic!("Expected AuthenticationError about expiration"),
//             }
//         }
//     }

//     #[tokio::test]
//     async fn test_extract_user_id() {
//         let req = Request::builder()
//             .method(Method::GET)
//             .uri("/test")
//             .body(Body::empty())
//             .unwrap();

//         // Test without user_id in extensions
//         let result = extract_user_id(&req);
//         assert!(result.is_err());

//         // Test with user_id in extensions
//         let mut req = Request::builder()
//             .method(Method::GET)
//             .uri("/test")
//             .body(Body::empty())
//             .unwrap();

//         req.extensions_mut().insert(123i64);

//         let result = extract_user_id(&req);
//         assert!(result.is_ok());
//         assert_eq!(result.unwrap(), 123);
//     }
// }
