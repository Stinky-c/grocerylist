use crate::ApiErrorResponse;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use utoipa::ToSchema;

#[derive(Debug)]
pub enum ApiError {
    Custom(StatusCode, ApiErrorResponse),
    // generic errors
    InternalServerError,
    Forbidden,
    Unauthorized,
    SessionExpired,
    BadRequest,

    /// Crate errors
    SqlxError(sqlx::Error),
    Argon2HashError(argon2::password_hash::Error),
}

impl ApiError {
    fn into_pair(self) -> (StatusCode, ApiErrorResponse) {
        match self {
            ApiError::Custom(code, body) => (code, body),
            ApiError::SqlxError(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ApiErrorResponse::INTERNAL_SERVER_ERROR(),
            ),
            ApiError::Argon2HashError(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ApiErrorResponse::INTERNAL_SERVER_ERROR(),
            ),
            ApiError::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ApiErrorResponse::INTERNAL_SERVER_ERROR(),
            ),
            ApiError::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                ApiErrorResponse::from("unauthorized"),
            ),
            ApiError::Forbidden => (
                StatusCode::FORBIDDEN,
                ApiErrorResponse::from("access is forbidden"),
            ),
            ApiError::SessionExpired => (
                StatusCode::UNAUTHORIZED,
                ApiErrorResponse::from("session expired"),
            ),
            ApiError::BadRequest => (
                StatusCode::BAD_REQUEST,
                ApiErrorResponse::from("bad request"),
            ),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (l, r) = self.into_pair();
        (l, r.json()).into_response()
    }
}
