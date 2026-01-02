use crate::schema::{auth_users, roles};
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Selectable, Debug, Clone)]
#[diesel(table_name = roles)]
pub struct Role {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Queryable, Selectable, Debug, Clone)]
#[diesel(table_name = auth_users)]
pub struct AuthUser {
    pub id: Uuid,
    pub username: String,
    pub password_hash: String,
    pub role_id: i32,
    pub is_active: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Debug, Clone)]
#[diesel(table_name = auth_users)]
pub struct NewAuthUser {
    pub username: String,
    pub password_hash: String,
    pub role_id: i32,
}
