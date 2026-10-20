use crate::error::body::ErrorBody;
use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("expired token")]
    ExpiredToken,

    #[error("validation failed: {0}")]
    Validation(String),

    #[error("resource not found: {0}")]
    NotFound(String),

    #[error("database error")]
    Database(#[source] sqlx::Error),

    #[error("unexpected internal error")]
    Internal(#[source] anyhow::Error),

    #[error("missing auth header")]
    MissingAuthHeader,

    #[error("invalid auth formatting")]
    Authorization,
}

impl AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::Validation(_) | AppError::MissingAuthHeader => StatusCode::BAD_REQUEST,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::Database(_) | Self::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::ExpiredToken | AppError::Authorization => StatusCode::UNAUTHORIZED,
        }
    }

    fn public_body(&self) -> ErrorBody {
        match self {
            Self::ExpiredToken => ErrorBody {
                code: "expire_token".into(),
                message: "Token is expired, please authenticate".into(),
                request_id: None,
            },
            Self::Validation(message) => ErrorBody {
                code: "validation_error".into(),
                message: message.clone(),
                request_id: None,
            },

            Self::NotFound(message) => ErrorBody {
                code: "not_found".into(),
                message: message.clone(),
                request_id: None,
            },
            Self::MissingAuthHeader => ErrorBody {
                code: "missing_auth".into(),
                message: "Missing expected authorization bearer".into(),
                request_id: None,
            },

            Self::Authorization => ErrorBody {
                code: "bad_auth".into(),
                message: "Bad authorization provided.".into(),
                request_id: None,
            },

            // Do not expose database or internal error details.
            Self::Database(_) | Self::Internal(_) => ErrorBody {
                code: "internal_error".into(),
                message: "An internal error occurred".into(),
                request_id: None,
            },
        }
    }
}

use tracing::error;

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = self.status_code();

        match &self {
            Self::Database(_) | Self::Internal(_) => {
                error!(error = ?self, "request failed");
            }

            _ => {
                tracing::debug!(error = ?self, "request rejected");
            }
        }

        (status, Json(self.public_body())).into_response()
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        Self::Database(err)
    }
}

impl From<argon2::password_hash::Error> for AppError {
    fn from(err: argon2::password_hash::Error) -> Self {
        Self::Internal(err.into())
    }
}

impl From<argon2::password_hash::phc::Error> for AppError {
    fn from(err: argon2::password_hash::phc::Error) -> Self {
        Self::Internal(err.into())
    }
}
