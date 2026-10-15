use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateUserRequest {
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateUserResponse {
    pub(crate) success: bool,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserDetailsResponse {
    username: String,
    email: String,
    created_at: chrono::DateTime<chrono::Utc>,
}
