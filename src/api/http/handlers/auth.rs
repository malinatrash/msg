use axum::{
    Extension, Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;

use crate::api::http::dto::{
    AuthResponse, ErrorResponse, LoginRequest, RegisterRequest, UserInfoResponse, UserResponse,
};
use crate::api::http::middleware::AuthUser;
use crate::api::http::state::AppState;
use crate::usecase::AuthError;

#[utoipa::path(
    post,
    path = "/auth/register",
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "User registered successfully", body = UserResponse),
        (status = 400, description = "Invalid input", body = ErrorResponse),
        (status = 409, description = "Username already exists", body = ErrorResponse),
    ),
    tag = "Authentication"
)]
pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> impl IntoResponse {
    match state
        .uc
        .auth
        .create_user(payload.username, payload.password, payload.role_id)
    {
        Ok(user) => (
            StatusCode::CREATED,
            Json(UserResponse::from(user)).into_response(),
        ),
        Err(e) => error_response(e),
    }
}

#[utoipa::path(
    post,
    path = "/auth/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = AuthResponse),
        (status = 401, description = "Invalid credentials", body = ErrorResponse),
        (status = 403, description = "User is deactivated", body = ErrorResponse),
    ),
    tag = "Authentication"
)]
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> impl IntoResponse {
    match state.uc.auth.login(payload.username, payload.password) {
        Ok(auth_response) => (
            StatusCode::OK,
            Json(AuthResponse {
                user: UserResponse::from(auth_response.user),
                token: auth_response.token,
            })
            .into_response(),
        ),
        Err(e) => error_response(e),
    }
}

#[utoipa::path(
    get,
    path = "/auth/me",
    responses(
        (status = 200, description = "Current user info", body = UserInfoResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 404, description = "User not found", body = ErrorResponse),
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Authentication"
)]
pub async fn me(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
) -> impl IntoResponse {
    match state.uc.auth.get_user_by_id(auth_user.user_id) {
        Ok(user_info) => (
            StatusCode::OK,
            Json(UserInfoResponse::from(user_info)).into_response(),
        ),
        Err(e) => error_response(e),
    }
}

#[utoipa::path(
    get,
    path = "/users/{user_id}",
    params(
        ("user_id" = Uuid, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User info retrieved", body = UserInfoResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 404, description = "User not found", body = ErrorResponse),
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Users"
)]
pub async fn get_user_by_id(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
    Extension(_auth_user): Extension<AuthUser>,
) -> impl IntoResponse {
    match state.uc.auth.get_user_by_id(user_id) {
        Ok(user_info) => (
            StatusCode::OK,
            Json(UserInfoResponse::from(user_info)).into_response(),
        ),
        Err(e) => error_response(e),
    }
}

fn error_response(err: AuthError) -> (StatusCode, axum::response::Response) {
    let (status, code) = match &err {
        AuthError::UsernameExists => (StatusCode::CONFLICT, "USERNAME_EXISTS"),
        AuthError::InvalidUsername(_) => (StatusCode::BAD_REQUEST, "INVALID_USERNAME"),
        AuthError::InvalidPassword(_) => (StatusCode::BAD_REQUEST, "INVALID_PASSWORD"),
        AuthError::UserNotFound => (StatusCode::NOT_FOUND, "USER_NOT_FOUND"),
        AuthError::InvalidCredentials => (StatusCode::UNAUTHORIZED, "INVALID_CREDENTIALS"),
        AuthError::UserDeactivated => (StatusCode::FORBIDDEN, "USER_DEACTIVATED"),
        AuthError::TokenGenerationFailed(_) => {
            (StatusCode::INTERNAL_SERVER_ERROR, "TOKEN_GENERATION_FAILED")
        }
        AuthError::TokenValidationFailed(_) => {
            (StatusCode::UNAUTHORIZED, "TOKEN_VALIDATION_FAILED")
        }
        AuthError::Internal(_) => (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR"),
    };

    (
        status,
        Json(ErrorResponse {
            error: err.to_string(),
            code: code.to_string(),
        })
        .into_response(),
    )
}
