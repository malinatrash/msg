use super::auth::repo::AuthRepository;
use super::chat::repo::ChatRepository;
use super::factory::Factory;
use crate::bootstrap::postgres::Postgres;
use std::sync::Arc;

pub struct Repository {
    pub auth: AuthRepository,
    pub chat: ChatRepository,
}

impl Repository {
    pub fn new(postgres: Arc<Postgres>) -> Self {
        let factory = Factory::new(postgres);

        Self {
            auth: factory.new_auth_repository(),
            chat: factory.new_chat_repository(),
        }
    }
}

impl Clone for Repository {
    fn clone(&self) -> Self {
        Self {
            auth: self.auth.clone(),
            chat: self.chat.clone(),
        }
    }
}
