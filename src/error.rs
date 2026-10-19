use crate::ApiErrorResponse;
use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use utoipa::IntoResponses;

#[derive(Debug, IntoResponses)]
#[response(content_type = "application/json")]
pub enum ApiError {
    // generic errors
    #[response(status = 500, description = "Internal Server Error")]
    InternalServerError,
    #[response(status = 403, description = "Forbidden")]
    Forbidden,
    #[response(status = 401, description = "Unauthorized")]
    Unauthorized,
    #[response(status = 401, description = "Authorization Expired")]
    SessionExpired,
    #[response(status = 400, description = "Malformed request")]
    BadRequest,

    /// Crate errors
    #[response(status = 500, description = "Internal Server Error")]
    SqlxError,
    #[response(status = 500, description = "Internal Server Error")]
    Argon2HashError,
}

impl ApiError {
    fn into_pair(self) -> (StatusCode, ApiErrorResponse) {
        match self {
            ApiError::SqlxError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ApiErrorResponse::INTERNAL_SERVER_ERROR(),
            ),
            ApiError::Argon2HashError => (
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

impl From<sqlx::Error> for ApiError {
    fn from(_: sqlx::Error) -> Self {
        Self::SqlxError
    }
}

pub type Result<T> = std::result::Result<Json<T>, impl IntoResponse>;
