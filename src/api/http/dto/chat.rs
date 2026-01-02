use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateChatRequest {
    #[schema(example = "My Secret Chat")]
    pub name: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct InviteUserRequest {
    #[schema(example = "john_doe")]
    pub username: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct SendMessageRequest {
    #[schema(example = "U2FsdGVkX1+vupppZksvRf5pq5g5XjFRIipRkwB0K1Y=")]
    pub encrypted_content: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct GetMessagesQuery {
    #[schema(example = 50)]
    #[serde(default = "default_limit")]
    pub limit: i64,
    #[schema(example = 0)]
    #[serde(default)]
    pub offset: i64,
}

fn default_limit() -> i64 {
    50
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ChatResponse {
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub id: Uuid,
    #[schema(example = "My Secret Chat")]
    pub name: String,
    #[schema(example = "550e8400-e29b-41d4-a716-446655440001")]
    pub created_by: Uuid,
    #[schema(example = "2024-01-02 12:00:00")]
    pub created_at: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct MessageResponse {
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub id: Uuid,
    #[schema(example = "550e8400-e29b-41d4-a716-446655440001")]
    pub chat_id: Uuid,
    #[schema(example = "550e8400-e29b-41d4-a716-446655440002")]
    pub sender_id: Uuid,
    #[schema(example = "U2FsdGVkX1+vupppZksvRf5pq5g5XjFRIipRkwB0K1Y=")]
    pub encrypted_content: String,
    #[schema(example = "2024-01-02 12:00:00")]
    pub created_at: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ChatMemberResponse {
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub user_id: Uuid,
    #[schema(example = "john_doe")]
    pub username: String,
    #[schema(example = "2024-01-02 12:00:00")]
    pub joined_at: String,
}

impl From<crate::usecase::ChatInfo> for ChatResponse {
    fn from(info: crate::usecase::ChatInfo) -> Self {
        Self {
            id: info.id,
            name: info.name,
            created_by: info.created_by,
            created_at: info.created_at,
        }
    }
}

impl From<crate::usecase::MessageInfo> for MessageResponse {
    fn from(info: crate::usecase::MessageInfo) -> Self {
        Self {
            id: info.id,
            chat_id: info.chat_id,
            sender_id: info.sender_id,
            encrypted_content: info.encrypted_content,
            created_at: info.created_at,
        }
    }
}

impl From<crate::usecase::ChatMemberInfo> for ChatMemberResponse {
    fn from(info: crate::usecase::ChatMemberInfo) -> Self {
        Self {
            user_id: info.user_id,
            username: info.username,
            joined_at: info.joined_at,
        }
    }
}
