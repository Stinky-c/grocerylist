use axum::Json;
use serde::Serialize;
use utoipa::ToSchema;

pub mod app;
pub mod auth;
pub mod config;
pub mod models;
pub mod routes;
pub mod state;
mod error;

#[derive(Debug, Serialize, ToSchema)]
pub struct ApiErrorResponse {
    message: String,
}

impl ApiErrorResponse {
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
