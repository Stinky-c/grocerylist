use axum::Json;
use serde::Serialize;
use utoipa::ToSchema;

pub mod app;
pub mod auth;
pub mod config;
mod error;
pub mod models;
pub mod routes;
pub mod state;

#[derive(Debug, Serialize, ToSchema)]
pub struct ApiErrorResponse {
    message: String,
}

impl ApiErrorResponse {
    pub fn INTERNAL_SERVER_ERROR() -> Self {
        ApiErrorResponse {
            message: "internal server error".to_string(),
        }
    }
    pub fn new(message: String) -> Self {
        ApiErrorResponse { message }
    }

    pub fn json(self) -> Json<Self> {
        Json(self)
    }
}

impl From<&str> for ApiErrorResponse {
    fn from(value: &str) -> Self {
        Self {
            message: value.to_string(),
        }
    }
}
