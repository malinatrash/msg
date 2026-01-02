use thiserror::Error;

#[derive(Debug, Error)]
pub enum ChatError {
    #[error("Chat not found")]
    ChatNotFound,

    #[error("User not found: {0}")]
    UserNotFound(String),

    #[error("Not a member of this chat")]
    NotMember,

    #[error("User is already a member")]
    AlreadyMember,

    #[error("Invalid chat name: {0}")]
    InvalidChatName(String),

    #[error("Internal error: {0}")]
    Internal(String),
}
