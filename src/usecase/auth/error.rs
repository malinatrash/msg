use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Username already exists")]
    UsernameExists,

    #[error("Invalid username: {0}")]
    InvalidUsername(String),

    #[error("Invalid password: {0}")]
    InvalidPassword(String),

    #[error("User not found")]
    UserNotFound,

    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("User is deactivated")]
    UserDeactivated,

    #[error("Token generation failed: {0}")]
    TokenGenerationFailed(String),

    #[error("Token validation failed: {0}")]
    TokenValidationFailed(String),

    #[error("Internal error: {0}")]
    Internal(String),
}
