use super::auth::service::AuthService;
use super::chat::service::ChatService;
use crate::config::jwt::JwtConfig;
use crate::repository::Repository;

pub(super) struct Factory {
    repo: Repository,
    jwt_config: JwtConfig,
}

impl Factory {
    pub(super) fn new(repo: Repository, jwt_config: JwtConfig) -> Self {
        Self { repo, jwt_config }
    }

    pub(super) fn create_auth_service(&self) -> AuthService {
        AuthService::new(self.repo.clone(), &self.jwt_config)
    }

    pub(super) fn create_chat_service(&self) -> ChatService {
        ChatService::new(self.repo.clone())
    }
}
