use super::auth::service::AuthService;
use super::chat::service::ChatService;
use super::factory::Factory;
use crate::config::jwt::JwtConfig;
use crate::repository::Repository;

pub struct Service {
    pub auth: AuthService,
    pub chat: ChatService,
}

impl Service {
    pub fn new(repo: Repository, jwt_config: JwtConfig) -> Self {
        let factory = Factory::new(repo, jwt_config);

        Self {
            auth: factory.create_auth_service(),
            chat: factory.create_chat_service(),
        }
    }
}

impl Clone for Service {
    fn clone(&self) -> Self {
        Self {
            auth: self.auth.clone(),
            chat: self.chat.clone(),
        }
    }
}
