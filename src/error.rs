use crate::ApiErrorResponse;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use std::error::Error;
use tracing::error;

pub enum AuthError {
    InvalidCredentials,
    Forbidden,
    Unauthorized,
    InternalServerError,
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
            AuthError::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ApiErrorResponse::from("An error occurred while processing your request.").json(),
            )
                .into_response(),
        }
    }
}
