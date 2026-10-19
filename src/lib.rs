use axum::Json;
use serde::Serialize;
use utoipa::ToSchema;

pub mod app;
pub mod auth;
pub mod config;
mod error;
pub mod models;
pub mod openapi;
pub mod routes;
pub mod state;
