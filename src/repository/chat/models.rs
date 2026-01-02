use chrono::NaiveDateTime;
use diesel::prelude::*;
use uuid::Uuid;

use crate::schema::{chat_members, chats, messages};

#[derive(Debug, Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = chats)]
pub struct Chat {
    pub id: Uuid,
    pub name: String,
    pub created_by: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = chats)]
pub struct NewChat {
    pub name: String,
    pub created_by: Uuid,
}

#[derive(Debug, Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = chat_members)]
pub struct ChatMember {
    pub id: Uuid,
    pub chat_id: Uuid,
    pub user_id: Uuid,
    pub invited_by: Option<Uuid>,
    pub joined_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = chat_members)]
pub struct NewChatMember {
    pub chat_id: Uuid,
    pub user_id: Uuid,
    pub invited_by: Option<Uuid>,
}

#[derive(Debug, Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = messages)]
pub struct Message {
    pub id: Uuid,
    pub chat_id: Uuid,
    pub sender_id: Uuid,
    pub encrypted_content: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = messages)]
pub struct NewMessage {
    pub chat_id: Uuid,
    pub sender_id: Uuid,
    pub encrypted_content: String,
}
