use super::auth::repo::AuthRepository;
use super::chat::repo::ChatRepository;
use crate::bootstrap::postgres::Postgres;
use std::sync::Arc;

pub(super) struct Factory {
    postgres: Arc<Postgres>,
}

impl Factory {
    pub(super) fn new(postgres: Arc<Postgres>) -> Self {
        Self { postgres }
    }

    pub(super) fn new_auth_repository(&self) -> AuthRepository {
        AuthRepository::new(self.postgres.clone())
    }

    pub(super) fn new_chat_repository(&self) -> ChatRepository {
        ChatRepository::new(self.postgres.clone())
    }
}
