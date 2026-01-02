use axum::{
    Extension, Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;

use crate::api::http::dto::{
    ChatMemberResponse, ChatResponse, CreateChatRequest, ErrorResponse, GetMessagesQuery,
    InviteUserRequest, MessageResponse, SendMessageRequest,
};
use crate::api::http::middleware::AuthUser;
use crate::api::http::state::AppState;
use crate::usecase::ChatError;

#[utoipa::path(
    post,
    path = "/chats",
    request_body = CreateChatRequest,
    responses(
        (status = 201, description = "Chat created successfully", body = ChatResponse),
        (status = 400, description = "Invalid input", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
    ),
    security(("bearer_auth" = [])),
    tag = "Chats"
)]
pub async fn create_chat(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Json(payload): Json<CreateChatRequest>,
) -> impl IntoResponse {
    match state.uc.chat.create_chat(payload.name, auth_user.user_id) {
        Ok(chat) => (
            StatusCode::CREATED,
            Json(ChatResponse::from(chat)).into_response(),
        ),
        Err(e) => error_response(e),
    }
}

#[utoipa::path(
    get,
    path = "/chats",
    responses(
        (status = 200, description = "List of user's chats", body = Vec<ChatResponse>),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
    ),
    security(("bearer_auth" = [])),
    tag = "Chats"
)]
pub async fn get_my_chats(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
) -> impl IntoResponse {
    match state.uc.chat.get_user_chats(auth_user.user_id) {
        Ok(chats) => (
            StatusCode::OK,
            Json(
                chats
                    .into_iter()
                    .map(ChatResponse::from)
                    .collect::<Vec<_>>(),
            )
            .into_response(),
        ),
        Err(e) => error_response(e),
    }
}

#[utoipa::path(
    get,
    path = "/chats/{chat_id}",
    params(("chat_id" = Uuid, Path, description = "Chat ID")),
    responses(
        (status = 200, description = "Chat details", body = ChatResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Not a member", body = ErrorResponse),
        (status = 404, description = "Chat not found", body = ErrorResponse),
    ),
    security(("bearer_auth" = [])),
    tag = "Chats"
)]
pub async fn get_chat(
    State(state): State<AppState>,
    Path(chat_id): Path<Uuid>,
    Extension(auth_user): Extension<AuthUser>,
) -> impl IntoResponse {
    match state.uc.chat.get_chat(chat_id, auth_user.user_id) {
        Ok(chat) => (
            StatusCode::OK,
            Json(ChatResponse::from(chat)).into_response(),
        ),
        Err(e) => error_response(e),
    }
}

#[utoipa::path(
    post,
    path = "/chats/{chat_id}/invite",
    params(("chat_id" = Uuid, Path, description = "Chat ID")),
    request_body = InviteUserRequest,
    responses(
        (status = 200, description = "User invited successfully"),
        (status = 400, description = "User already a member", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Not a member", body = ErrorResponse),
        (status = 404, description = "User not found", body = ErrorResponse),
    ),
    security(("bearer_auth" = [])),
    tag = "Chats"
)]
pub async fn invite_user(
    State(state): State<AppState>,
    Path(chat_id): Path<Uuid>,
    Extension(auth_user): Extension<AuthUser>,
    Json(payload): Json<InviteUserRequest>,
) -> impl IntoResponse {
    match state
        .uc
        .chat
        .invite_user_by_username(chat_id, payload.username, auth_user.user_id)
    {
        Ok(()) => (
            StatusCode::OK,
            Json(serde_json::json!({"message": "User invited successfully"})).into_response(),
        ),
        Err(e) => error_response(e),
    }
}

#[utoipa::path(
    get,
    path = "/chats/{chat_id}/members",
    params(("chat_id" = Uuid, Path, description = "Chat ID")),
    responses(
        (status = 200, description = "List of chat members", body = Vec<ChatMemberResponse>),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Not a member", body = ErrorResponse),
    ),
    security(("bearer_auth" = [])),
    tag = "Chats"
)]
pub async fn get_chat_members(
    State(state): State<AppState>,
    Path(chat_id): Path<Uuid>,
    Extension(auth_user): Extension<AuthUser>,
) -> impl IntoResponse {
    match state.uc.chat.get_chat_members(chat_id, auth_user.user_id) {
        Ok(members) => (
            StatusCode::OK,
            Json(
                members
                    .into_iter()
                    .map(ChatMemberResponse::from)
                    .collect::<Vec<_>>(),
            )
            .into_response(),
        ),
        Err(e) => error_response(e),
    }
}

#[utoipa::path(
    post,
    path = "/chats/{chat_id}/messages",
    params(("chat_id" = Uuid, Path, description = "Chat ID")),
    request_body = SendMessageRequest,
    responses(
        (status = 201, description = "Message sent successfully", body = MessageResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Not a member", body = ErrorResponse),
    ),
    security(("bearer_auth" = [])),
    tag = "Messages"
)]
pub async fn send_message(
    State(state): State<AppState>,
    Path(chat_id): Path<Uuid>,
    Extension(auth_user): Extension<AuthUser>,
    Json(payload): Json<SendMessageRequest>,
) -> impl IntoResponse {
    match state
        .uc
        .chat
        .send_message(chat_id, auth_user.user_id, payload.encrypted_content)
    {
        Ok(message) => (
            StatusCode::CREATED,
            Json(MessageResponse::from(message)).into_response(),
        ),
        Err(e) => error_response(e),
    }
}

#[utoipa::path(
    get,
    path = "/chats/{chat_id}/messages",
    params(
        ("chat_id" = Uuid, Path, description = "Chat ID"),
        ("limit" = Option<i64>, Query, description = "Number of messages to return"),
        ("offset" = Option<i64>, Query, description = "Offset for pagination"),
    ),
    responses(
        (status = 200, description = "List of messages", body = Vec<MessageResponse>),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Not a member", body = ErrorResponse),
    ),
    security(("bearer_auth" = [])),
    tag = "Messages"
)]
pub async fn get_messages(
    State(state): State<AppState>,
    Path(chat_id): Path<Uuid>,
    Query(query): Query<GetMessagesQuery>,
    Extension(auth_user): Extension<AuthUser>,
) -> impl IntoResponse {
    match state
        .uc
        .chat
        .get_messages(chat_id, auth_user.user_id, query.limit, query.offset)
    {
        Ok(messages) => (
            StatusCode::OK,
            Json(
                messages
                    .into_iter()
                    .map(MessageResponse::from)
                    .collect::<Vec<_>>(),
            )
            .into_response(),
        ),
        Err(e) => error_response(e),
    }
}

fn error_response(err: ChatError) -> (StatusCode, axum::response::Response) {
    let (status, code) = match &err {
        ChatError::ChatNotFound => (StatusCode::NOT_FOUND, "CHAT_NOT_FOUND"),
        ChatError::UserNotFound(_) => (StatusCode::NOT_FOUND, "USER_NOT_FOUND"),
        ChatError::NotMember => (StatusCode::FORBIDDEN, "NOT_MEMBER"),
        ChatError::AlreadyMember => (StatusCode::BAD_REQUEST, "ALREADY_MEMBER"),
        ChatError::InvalidChatName(_) => (StatusCode::BAD_REQUEST, "INVALID_CHAT_NAME"),
        ChatError::Internal(_) => (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR"),
    };

    (
        status,
        Json(ErrorResponse {
            error: err.to_string(),
            code: code.to_string(),
        })
        .into_response(),
    )
}
