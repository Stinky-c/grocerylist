use crate::error::AuthError;
use crate::models::db::users::User;
use axum::http::Response;
use axum::response::{ErrorResponse, IntoResponse};
use chrono::Duration;
use sqlx::{Error, PgPool};
use uuid::Uuid;

#[derive(serde::Deserialize, Clone)]
pub struct Claim {
    user_id: Uuid,
    // https://www.rfc-editor.org/info/rfc7519/#section-4.1
    pub jti: Uuid, // Used for session cache lookups
    pub iss: String,
    pub iat: u64,
    pub exp: u64,
}

const JWT_EXPIRY_SECS: u64 = Duration::days(7).num_seconds() as u64;

impl Claim {
    pub fn new(user_id: Uuid) -> Self {
        let now = jsonwebtoken::get_current_timestamp();
        Self {
            user_id,
            jti: Uuid::new_v4(),
            iss: env!("CARGO_CRATE_NAME").to_string(),
            iat: now,
            exp: now + JWT_EXPIRY_SECS,
        }
    }

    pub(super) fn valid(&self) -> bool {
        let now = jsonwebtoken::get_current_timestamp();
        now > self.exp && self.iss == env!("CARGO_CRATE_NAME")
    }

    pub async fn get_user(&self, pool: &PgPool) -> Result<User, ErrorResponse> {
        let v = User::by_id(self.user_id, pool)
            .await
            .map_err(|_| AuthError::InternalServerError)?;

        v.ok_or_else(|| panic!("TODO:  Handle missing user from database"))
    }
}
