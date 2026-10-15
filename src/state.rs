use axum::extract::{FromRef, State};
use std::sync::Arc;

pub type AxumApiState = Arc<ApiState>;

// Make sure everything inside is cheap to clone
#[derive(Debug, Clone)]
pub struct ApiState {
    pub pool: sqlx::postgres::PgPool,

    pub jwt_decode: Arc<jsonwebtoken::DecodingKey>,
    pub jwt_encode: Arc<jsonwebtoken::EncodingKey>,
    pub jwt_validation: Arc<jsonwebtoken::Validation>,
}
