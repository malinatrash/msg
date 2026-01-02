use diesel::prelude::*;
use std::sync::Arc;
use uuid::Uuid;

use super::models::{Chat, ChatMember, Message, NewChat, NewChatMember, NewMessage};
use crate::bootstrap::postgres::Postgres;
use crate::schema::{chat_members, chats, messages};

#[derive(Clone)]
pub struct ChatRepository {
    postgres: Arc<Postgres>,
}

impl ChatRepository {
    pub fn new(postgres: Arc<Postgres>) -> Self {
        Self { postgres }
    }

    pub fn create_chat(&self, name: String, created_by: Uuid) -> Result<Chat, String> {
        let mut conn = self.postgres.conn()?;

        let new_chat = NewChat { name, created_by };

        diesel::insert_into(chats::table)
            .values(&new_chat)
            .returning(Chat::as_returning())
            .get_result(&mut conn)
            .map_err(|e| format!("Failed to create chat: {}", e))
    }

    pub fn find_chat_by_id(&self, chat_id: Uuid) -> Result<Option<Chat>, String> {
        let mut conn = self.postgres.conn()?;

        chats::table
            .filter(chats::id.eq(chat_id))
            .first::<Chat>(&mut conn)
            .optional()
            .map_err(|e| format!("Failed to find chat: {}", e))
    }

    pub fn get_user_chats(&self, user_id: Uuid) -> Result<Vec<Chat>, String> {
        let mut conn = self.postgres.conn()?;

        chat_members::table
            .inner_join(chats::table.on(chats::id.eq(chat_members::chat_id)))
            .filter(chat_members::user_id.eq(user_id))
            .select(Chat::as_select())
            .load::<Chat>(&mut conn)
            .map_err(|e| format!("Failed to get user chats: {}", e))
    }

    pub fn add_member(
        &self,
        chat_id: Uuid,
        user_id: Uuid,
        invited_by: Option<Uuid>,
    ) -> Result<ChatMember, String> {
        let mut conn = self.postgres.conn()?;

        let new_member = NewChatMember {
            chat_id,
            user_id,
            invited_by,
        };

        diesel::insert_into(chat_members::table)
            .values(&new_member)
            .returning(ChatMember::as_returning())
            .get_result(&mut conn)
            .map_err(|e| format!("Failed to add member: {}", e))
    }

    pub fn is_member(&self, chat_id: Uuid, user_id: Uuid) -> Result<bool, String> {
        let mut conn = self.postgres.conn()?;

        let count: i64 = chat_members::table
            .filter(chat_members::chat_id.eq(chat_id))
            .filter(chat_members::user_id.eq(user_id))
            .count()
            .get_result(&mut conn)
            .map_err(|e| format!("Failed to check membership: {}", e))?;

        Ok(count > 0)
    }

    pub fn get_chat_members(&self, chat_id: Uuid) -> Result<Vec<ChatMember>, String> {
        let mut conn = self.postgres.conn()?;

        chat_members::table
            .filter(chat_members::chat_id.eq(chat_id))
            .load::<ChatMember>(&mut conn)
            .map_err(|e| format!("Failed to get chat members: {}", e))
    }

    pub fn create_message(
        &self,
        chat_id: Uuid,
        sender_id: Uuid,
        encrypted_content: String,
    ) -> Result<Message, String> {
        let mut conn = self.postgres.conn()?;

        let new_message = NewMessage {
            chat_id,
            sender_id,
            encrypted_content,
        };

        diesel::insert_into(messages::table)
            .values(&new_message)
            .returning(Message::as_returning())
            .get_result(&mut conn)
            .map_err(|e| format!("Failed to create message: {}", e))
    }

    pub fn get_chat_messages(
        &self,
        chat_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Message>, String> {
        let mut conn = self.postgres.conn()?;

        messages::table
            .filter(messages::chat_id.eq(chat_id))
            .order(messages::created_at.desc())
            .limit(limit)
            .offset(offset)
            .load::<Message>(&mut conn)
            .map_err(|e| format!("Failed to get messages: {}", e))
    }

    pub fn get_message_by_id(&self, message_id: Uuid) -> Result<Option<Message>, String> {
        let mut conn = self.postgres.conn()?;

        messages::table
            .filter(messages::id.eq(message_id))
            .first::<Message>(&mut conn)
            .optional()
            .map_err(|e| format!("Failed to find message: {}", e))
    }
}
