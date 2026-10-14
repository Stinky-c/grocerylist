use crate::state::AxumApiState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

#[utoipa::path(get, path = "/auth/users/list", responses(
    (status = 200, description = "Hello World!"),
    (status = 418, description = "Bad request")
))]
pub async fn list_users(State(state): AxumApiState) {}

#[utoipa::path(post, path = "/auth/create")]
pub async fn create_user(State(state): AxumApiState) -> impl IntoResponse {
    // sqlx::query!()
    (StatusCode::FORBIDDEN, "Not available")
}
