use uuid::Uuid;

use super::error::AuthError;
use super::jwt::JwtService;
use crate::config::jwt::JwtConfig;
use crate::repository::Repository;
use crate::repository::auth::{AuthUser, NewAuthUser};

const MIN_USERNAME_LENGTH: usize = 3;
const MAX_USERNAME_LENGTH: usize = 100;
const MIN_PASSWORD_LENGTH: usize = 8;

pub struct AuthService {
    repo: Repository,
    jwt: JwtService,
}

#[derive(Debug, Clone)]
pub struct AuthResponse {
    pub user: AuthUser,
    pub token: String,
}

#[derive(Debug, Clone)]
pub struct UserInfo {
    pub id: Uuid,
    pub username: String,
    pub role_id: i32,
    pub role_name: String,
    pub is_active: bool,
    pub created_at: chrono::NaiveDateTime,
}

impl AuthService {
    pub fn new(repo: Repository, jwt_config: &JwtConfig) -> Self {
        let jwt = JwtService::new(jwt_config.secret.clone(), jwt_config.expiration_hours);
        Self { repo, jwt }
    }

    #[tracing::instrument(skip(self, password))]
    pub fn create_user(
        &self,
        username: String,
        password: String,
        role_id: Option<i32>,
    ) -> Result<AuthUser, AuthError> {
        self.validate_username(&username)?;
        self.validate_password(&password)?;

        if self
            .repo
            .auth
            .username_exists(&username)
            .map_err(|e| AuthError::Internal(e))?
        {
            return Err(AuthError::UsernameExists);
        }

        let password_hash = bcrypt::hash(&password, bcrypt::DEFAULT_COST)
            .map_err(|e| AuthError::Internal(e.to_string()))?;

        let new_user = NewAuthUser {
            username,
            password_hash,
            role_id: role_id.unwrap_or(1),
        };

        self.repo
            .auth
            .create_user(new_user)
            .map_err(|e| AuthError::Internal(e))
    }

    #[tracing::instrument(skip(self, password))]
    pub fn login(&self, username: String, password: String) -> Result<AuthResponse, AuthError> {
        let user = self
            .repo
            .auth
            .find_by_username(&username)
            .map_err(|_| AuthError::InvalidCredentials)?;

        if !user.is_active {
            return Err(AuthError::UserDeactivated);
        }

        let valid = bcrypt::verify(&password, &user.password_hash)
            .map_err(|e| AuthError::Internal(e.to_string()))?;

        if !valid {
            return Err(AuthError::InvalidCredentials);
        }

        let token = self.jwt.generate_token(user.id, user.role_id)?;

        Ok(AuthResponse { user, token })
    }

    #[tracing::instrument(skip(self))]
    pub fn get_user_by_id(&self, user_id: Uuid) -> Result<UserInfo, AuthError> {
        let user = self
            .repo
            .auth
            .find_by_id(user_id)
            .map_err(|_| AuthError::UserNotFound)?;

        let role = self
            .repo
            .auth
            .find_role_by_id(user.role_id)
            .map_err(|e| AuthError::Internal(e))?;

        Ok(UserInfo {
            id: user.id,
            username: user.username,
            role_id: user.role_id,
            role_name: role.name,
            is_active: user.is_active,
            created_at: user.created_at,
        })
    }

    #[tracing::instrument(skip(self))]
    pub fn validate_token(&self, token: &str) -> Result<(Uuid, i32), AuthError> {
        let claims = self.jwt.validate_token(token)?;
        let user_id = Uuid::parse_str(&claims.user_id)
            .map_err(|e| AuthError::TokenValidationFailed(e.to_string()))?;
        Ok((user_id, claims.role_id))
    }

    fn validate_username(&self, username: &str) -> Result<(), AuthError> {
        if username.len() < MIN_USERNAME_LENGTH {
            return Err(AuthError::InvalidUsername(format!(
                "Username must be at least {} characters",
                MIN_USERNAME_LENGTH
            )));
        }
        if username.len() > MAX_USERNAME_LENGTH {
            return Err(AuthError::InvalidUsername(format!(
                "Username must be at most {} characters",
                MAX_USERNAME_LENGTH
            )));
        }
        if !username
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
        {
            return Err(AuthError::InvalidUsername(
                "Username can only contain alphanumeric characters, underscores, and hyphens"
                    .to_string(),
            ));
        }
        Ok(())
    }

    fn validate_password(&self, password: &str) -> Result<(), AuthError> {
        if password.len() < MIN_PASSWORD_LENGTH {
            return Err(AuthError::InvalidPassword(format!(
                "Password must be at least {} characters",
                MIN_PASSWORD_LENGTH
            )));
        }
        Ok(())
    }
}

impl Clone for AuthService {
    fn clone(&self) -> Self {
        Self {
            repo: self.repo.clone(),
            jwt: self.jwt.clone(),
        }
    }
}
