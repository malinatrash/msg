use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, ToSchema)]
pub struct RegisterRequest {
    #[schema(example = "john_doe")]
    pub username: String,
    #[schema(example = "SecurePass123")]
    pub password: String,
    #[serde(default)]
    #[schema(example = 1)]
    pub role_id: Option<i32>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct LoginRequest {
    #[schema(example = "john_doe")]
    pub username: String,
    #[schema(example = "SecurePass123")]
    pub password: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AuthResponse {
    pub user: UserResponse,
    #[schema(example = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...")]
    pub token: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UserResponse {
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub id: Uuid,
    #[schema(example = "john_doe")]
    pub username: String,
    #[schema(example = 1)]
    pub role_id: i32,
    #[schema(example = true)]
    pub is_active: bool,
    #[schema(example = "2024-01-02 12:00:00")]
    pub created_at: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UserInfoResponse {
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub id: Uuid,
    #[schema(example = "john_doe")]
    pub username: String,
    #[schema(example = 1)]
    pub role_id: i32,
    #[schema(example = "user")]
    pub role_name: String,
    #[schema(example = true)]
    pub is_active: bool,
    #[schema(example = "2024-01-02 12:00:00")]
    pub created_at: String,
}

impl From<crate::repository::auth::AuthUser> for UserResponse {
    fn from(user: crate::repository::auth::AuthUser) -> Self {
        Self {
            id: user.id,
            username: user.username,
            role_id: user.role_id,
            is_active: user.is_active,
            created_at: user.created_at.to_string(),
        }
    }
}

impl From<crate::usecase::UserInfo> for UserInfoResponse {
    fn from(info: crate::usecase::UserInfo) -> Self {
        Self {
            id: info.id,
            username: info.username,
            role_id: info.role_id,
            role_name: info.role_name,
            is_active: info.is_active,
            created_at: info.created_at.to_string(),
        }
    }
}
