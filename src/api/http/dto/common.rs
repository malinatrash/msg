use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct ErrorResponse {
    #[schema(example = "Invalid username")]
    pub error: String,
    #[schema(example = "INVALID_USERNAME")]
    pub code: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct MessageResponse {
    #[schema(example = "Operation successful")]
    pub message: String,
}
