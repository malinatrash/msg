use uuid::Uuid;

use super::error::ChatError;
use crate::repository::Repository;
use crate::repository::chat::{Chat, Message};

#[derive(Clone)]
pub struct ChatService {
    repo: Repository,
}

pub struct ChatInfo {
    pub id: Uuid,
    pub name: String,
    pub created_by: Uuid,
    pub created_at: String,
}

pub struct MessageInfo {
    pub id: Uuid,
    pub chat_id: Uuid,
    pub sender_id: Uuid,
    pub encrypted_content: String,
    pub created_at: String,
}

pub struct ChatMemberInfo {
    pub user_id: Uuid,
    pub username: String,
    pub joined_at: String,
}

impl ChatService {
    pub fn new(repo: Repository) -> Self {
        Self { repo }
    }

    pub fn create_chat(&self, name: String, creator_id: Uuid) -> Result<ChatInfo, ChatError> {
        if name.trim().is_empty() {
            return Err(ChatError::InvalidChatName(
                "Chat name cannot be empty".to_string(),
            ));
        }

        if name.len() > 255 {
            return Err(ChatError::InvalidChatName("Chat name too long".to_string()));
        }

        let chat = self
            .repo
            .chat
            .create_chat(name, creator_id)
            .map_err(|e| ChatError::Internal(e))?;

        self.repo
            .chat
            .add_member(chat.id, creator_id, None)
            .map_err(|e| ChatError::Internal(e))?;

        Ok(ChatInfo::from(chat))
    }

    pub fn get_user_chats(&self, user_id: Uuid) -> Result<Vec<ChatInfo>, ChatError> {
        let chats = self
            .repo
            .chat
            .get_user_chats(user_id)
            .map_err(|e| ChatError::Internal(e))?;

        Ok(chats.into_iter().map(ChatInfo::from).collect())
    }

    pub fn get_chat(&self, chat_id: Uuid, user_id: Uuid) -> Result<ChatInfo, ChatError> {
        let is_member = self
            .repo
            .chat
            .is_member(chat_id, user_id)
            .map_err(|e| ChatError::Internal(e))?;

        if !is_member {
            return Err(ChatError::NotMember);
        }

        let chat = self
            .repo
            .chat
            .find_chat_by_id(chat_id)
            .map_err(|e| ChatError::Internal(e))?
            .ok_or(ChatError::ChatNotFound)?;

        Ok(ChatInfo::from(chat))
    }

    pub fn invite_user_by_username(
        &self,
        chat_id: Uuid,
        username: String,
        inviter_id: Uuid,
    ) -> Result<(), ChatError> {
        let is_member = self
            .repo
            .chat
            .is_member(chat_id, inviter_id)
            .map_err(|e| ChatError::Internal(e))?;

        if !is_member {
            return Err(ChatError::NotMember);
        }

        let user = self
            .repo
            .auth
            .find_by_username(&username)
            .map_err(|_| ChatError::UserNotFound(username.clone()))?;

        let already_member = self
            .repo
            .chat
            .is_member(chat_id, user.id)
            .map_err(|e| ChatError::Internal(e))?;

        if already_member {
            return Err(ChatError::AlreadyMember);
        }

        self.repo
            .chat
            .add_member(chat_id, user.id, Some(inviter_id))
            .map_err(|e| ChatError::Internal(e))?;

        Ok(())
    }

    pub fn get_chat_members(
        &self,
        chat_id: Uuid,
        user_id: Uuid,
    ) -> Result<Vec<ChatMemberInfo>, ChatError> {
        let is_member = self
            .repo
            .chat
            .is_member(chat_id, user_id)
            .map_err(|e| ChatError::Internal(e))?;

        if !is_member {
            return Err(ChatError::NotMember);
        }

        let members = self
            .repo
            .chat
            .get_chat_members(chat_id)
            .map_err(|e| ChatError::Internal(e))?;

        let mut result = Vec::new();
        for member in members {
            if let Ok(user) = self.repo.auth.find_by_id(member.user_id) {
                result.push(ChatMemberInfo {
                    user_id: member.user_id,
                    username: user.username,
                    joined_at: member.joined_at.format("%Y-%m-%d %H:%M:%S").to_string(),
                });
            }
        }

        Ok(result)
    }

    pub fn send_message(
        &self,
        chat_id: Uuid,
        sender_id: Uuid,
        encrypted_content: String,
    ) -> Result<MessageInfo, ChatError> {
        let is_member = self
            .repo
            .chat
            .is_member(chat_id, sender_id)
            .map_err(|e| ChatError::Internal(e))?;

        if !is_member {
            return Err(ChatError::NotMember);
        }

        let message = self
            .repo
            .chat
            .create_message(chat_id, sender_id, encrypted_content)
            .map_err(|e| ChatError::Internal(e))?;

        Ok(MessageInfo::from(message))
    }

    pub fn get_messages(
        &self,
        chat_id: Uuid,
        user_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<MessageInfo>, ChatError> {
        let is_member = self
            .repo
            .chat
            .is_member(chat_id, user_id)
            .map_err(|e| ChatError::Internal(e))?;

        if !is_member {
            return Err(ChatError::NotMember);
        }

        let messages = self
            .repo
            .chat
            .get_chat_messages(chat_id, limit, offset)
            .map_err(|e| ChatError::Internal(e))?;

        Ok(messages.into_iter().map(MessageInfo::from).collect())
    }
}

impl From<Chat> for ChatInfo {
    fn from(chat: Chat) -> Self {
        Self {
            id: chat.id,
            name: chat.name,
            created_by: chat.created_by,
            created_at: chat.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
        }
    }
}

impl From<Message> for MessageInfo {
    fn from(msg: Message) -> Self {
        Self {
            id: msg.id,
            chat_id: msg.chat_id,
            sender_id: msg.sender_id,
            encrypted_content: msg.encrypted_content,
            created_at: msg.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
        }
    }
}
