use axum::{
    Router, middleware,
    routing::{get, post},
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use super::handlers::{auth, chat, health};
use super::middleware::auth_middleware;
use super::openapi::ApiDoc;
use super::state::AppState;

pub fn create_router(state: AppState) -> Router {
    let public_routes = Router::new()
        .route("/health", get(health::health))
        .route("/auth/register", post(auth::register))
        .route("/auth/login", post(auth::login));

    let protected_routes = Router::new()
        .route("/auth/me", get(auth::me))
        .route("/users/:user_id", get(auth::get_user_by_id))
        .route("/chats", post(chat::create_chat))
        .route("/chats", get(chat::get_my_chats))
        .route("/chats/:chat_id", get(chat::get_chat))
        .route("/chats/:chat_id/invite", post(chat::invite_user))
        .route("/chats/:chat_id/members", get(chat::get_chat_members))
        .route("/chats/:chat_id/messages", post(chat::send_message))
        .route("/chats/:chat_id/messages", get(chat::get_messages))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ));

    Router::new()
        .merge(SwaggerUi::new("/swagger").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .merge(public_routes)
        .merge(protected_routes)
        .with_state(state)
}
