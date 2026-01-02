use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode, header},
    middleware::Next,
    response::Response,
};
use uuid::Uuid;

use crate::api::http::state::AppState;

#[derive(Clone, Debug)]
pub struct AuthUser {
    pub user_id: Uuid,
    pub role_id: i32,
}

pub async fn auth_middleware(
    State(state): State<AppState>,
    mut request: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok());

    let token = match auth_header {
        Some(h) if h.starts_with("Bearer ") => &h[7..],
        _ => return Err(StatusCode::UNAUTHORIZED),
    };

    let (user_id, role_id) = state
        .uc
        .auth
        .validate_token(token)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    request
        .extensions_mut()
        .insert(AuthUser { user_id, role_id });

    Ok(next.run(request).await)
}
