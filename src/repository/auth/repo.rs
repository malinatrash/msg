use super::models::{AuthUser, NewAuthUser, Role};
use crate::bootstrap::postgres::Postgres;
use crate::schema::{auth_users, roles};
use diesel::prelude::*;
use std::sync::Arc;
use uuid::Uuid;

pub struct AuthRepository {
    postgres: Arc<Postgres>,
}

impl AuthRepository {
    pub fn new(postgres: Arc<Postgres>) -> Self {
        Self { postgres }
    }

    #[tracing::instrument(skip(self, new_user), fields(username = %new_user.username))]
    pub fn create_user(&self, new_user: NewAuthUser) -> Result<AuthUser, String> {
        let mut conn = self.postgres.conn()?;

        diesel::insert_into(auth_users::table)
            .values(&new_user)
            .get_result(&mut conn)
            .map_err(|e| format!("Failed to create auth user: {}", e))
    }

    #[tracing::instrument(skip(self))]
    pub fn find_by_id(&self, user_id: Uuid) -> Result<AuthUser, String> {
        let mut conn = self.postgres.conn()?;

        auth_users::table
            .find(user_id)
            .first(&mut conn)
            .map_err(|e| format!("Failed to find auth user: {}", e))
    }

    #[tracing::instrument(skip(self))]
    pub fn find_by_username(&self, username: &str) -> Result<AuthUser, String> {
        let mut conn = self.postgres.conn()?;

        auth_users::table
            .filter(auth_users::username.eq(username))
            .first(&mut conn)
            .map_err(|e| format!("Failed to find auth user by username: {}", e))
    }

    #[tracing::instrument(skip(self))]
    pub fn username_exists(&self, username: &str) -> Result<bool, String> {
        let mut conn = self.postgres.conn()?;

        let count: i64 = auth_users::table
            .filter(auth_users::username.eq(username))
            .count()
            .get_result(&mut conn)
            .map_err(|e| format!("Failed to check username existence: {}", e))?;

        Ok(count > 0)
    }

    #[tracing::instrument(skip(self))]
    pub fn find_role_by_id(&self, role_id: i32) -> Result<Role, String> {
        let mut conn = self.postgres.conn()?;

        roles::table
            .find(role_id)
            .first(&mut conn)
            .map_err(|e| format!("Failed to find role: {}", e))
    }

    #[tracing::instrument(skip(self))]
    pub fn find_all_roles(&self) -> Result<Vec<Role>, String> {
        let mut conn = self.postgres.conn()?;

        roles::table
            .load(&mut conn)
            .map_err(|e| format!("Failed to load roles: {}", e))
    }

    #[tracing::instrument(skip(self))]
    pub fn update_user_role(&self, user_id: Uuid, new_role_id: i32) -> Result<AuthUser, String> {
        let mut conn = self.postgres.conn()?;

        diesel::update(auth_users::table.find(user_id))
            .set((
                auth_users::role_id.eq(new_role_id),
                auth_users::updated_at.eq(chrono::Utc::now().naive_utc()),
            ))
            .get_result(&mut conn)
            .map_err(|e| format!("Failed to update user role: {}", e))
    }

    #[tracing::instrument(skip(self))]
    pub fn deactivate_user(&self, user_id: Uuid) -> Result<AuthUser, String> {
        let mut conn = self.postgres.conn()?;

        diesel::update(auth_users::table.find(user_id))
            .set((
                auth_users::is_active.eq(false),
                auth_users::updated_at.eq(chrono::Utc::now().naive_utc()),
            ))
            .get_result(&mut conn)
            .map_err(|e| format!("Failed to deactivate user: {}", e))
    }
}

impl Clone for AuthRepository {
    fn clone(&self) -> Self {
        Self {
            postgres: self.postgres.clone(),
        }
    }
}
