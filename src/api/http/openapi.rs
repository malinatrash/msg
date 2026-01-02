use utoipa::OpenApi;

use super::dto::{
    AuthResponse, ChatMemberResponse, ChatResponse, CreateChatRequest, ErrorResponse,
    GetMessagesQuery, InviteUserRequest, LoginRequest, MessageResponse, RegisterRequest,
    SendMessageRequest, UserInfoResponse, UserResponse,
};

#[derive(OpenApi)]
#[openapi(
    paths(
        super::handlers::auth::register,
        super::handlers::auth::login,
        super::handlers::auth::me,
        super::handlers::auth::get_user_by_id,
        super::handlers::chat::create_chat,
        super::handlers::chat::get_my_chats,
        super::handlers::chat::get_chat,
        super::handlers::chat::invite_user,
        super::handlers::chat::get_chat_members,
        super::handlers::chat::send_message,
        super::handlers::chat::get_messages,
    ),
    components(
        schemas(
            RegisterRequest,
            LoginRequest,
            AuthResponse,
            UserResponse,
            UserInfoResponse,
            ErrorResponse,
            CreateChatRequest,
            InviteUserRequest,
            SendMessageRequest,
            GetMessagesQuery,
            ChatResponse,
            MessageResponse,
            ChatMemberResponse,
        )
    ),
    tags(
        (name = "Authentication", description = "Authentication endpoints"),
        (name = "Users", description = "User management endpoints"),
        (name = "Chats", description = "Chat management endpoints"),
        (name = "Messages", description = "Message endpoints")
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_auth",
                utoipa::openapi::security::SecurityScheme::Http(
                    utoipa::openapi::security::HttpBuilder::new()
                        .scheme(utoipa::openapi::security::HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            )
        }
    }
}
