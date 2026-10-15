use crate::ApiErrorResponse;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub enum AuthError {
    InvalidCredentials,
    Forbidden,
    Unauthorized,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        match self {
            AuthError::Forbidden => (
                StatusCode::FORBIDDEN,
                ApiErrorResponse::from("forbidden").json(),
            )
                .into_response(),
            AuthError::InvalidCredentials => (
                StatusCode::FORBIDDEN,
                ApiErrorResponse::from("invalid credentials").json(),
            )
                .into_response(),
            AuthError::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                ApiErrorResponse::from("not authorized").json(),
            )
                .into_response(),
        }
    }
}