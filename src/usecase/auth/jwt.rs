use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::error::AuthError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub user_id: String,
    pub role_id: i32,
    pub exp: i64,
    pub iat: i64,
}

pub struct JwtService {
    secret: String,
    expiration_hours: i64,
}

impl JwtService {
    pub fn new(secret: String, expiration_hours: i64) -> Self {
        Self {
            secret,
            expiration_hours,
        }
    }

    pub fn generate_token(&self, user_id: Uuid, role_id: i32) -> Result<String, AuthError> {
        let now = chrono::Utc::now();
        let exp = now + chrono::Duration::hours(self.expiration_hours);

        let claims = Claims {
            sub: user_id.to_string(),
            user_id: user_id.to_string(),
            role_id,
            exp: exp.timestamp(),
            iat: now.timestamp(),
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
        .map_err(|e| AuthError::TokenGenerationFailed(e.to_string()))
    }

    pub fn validate_token(&self, token: &str) -> Result<Claims, AuthError> {
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &Validation::default(),
        )
        .map(|data| data.claims)
        .map_err(|e| AuthError::TokenValidationFailed(e.to_string()))
    }
}

impl Clone for JwtService {
    fn clone(&self) -> Self {
        Self {
            secret: self.secret.clone(),
            expiration_hours: self.expiration_hours,
        }
    }
}
