use crate::ApiErrorResponse;
use crate::auth::Jwt;
use crate::models::http::users::{CreateUserRequest, CreateUserResponse};
use crate::state::AxumApiState;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response, Result};

#[utoipa::path(get, path = "/auth/users/list", responses(
    (status = 200, description = "Hello World!"),
    (status = 418, description = "Bad request")
))]
#[axum::debug_handler]
pub async fn list_users(Jwt(claim): Jwt, State(state): State<AxumApiState>) {}

#[utoipa::path(post, path = "/auth/create")]
#[axum::debug_handler]
pub async fn create_user(
    State(state): State<AxumApiState>,
    Json(req): Json<CreateUserRequest>,
) -> Result<Json<CreateUserResponse>> {
    Err(AuthError::Forbidden.into())
}

enum AuthError {
    Forbidden,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        match self {
            AuthError::Forbidden => (
                StatusCode::FORBIDDEN,
                ApiErrorResponse::from("forbidden").json(),
            )
                .into_response(),
        }
    }
}
