use crate::models::db::users::User;
use chrono::Duration;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

/// https://www.rfc-editor.org/info/rfc7519/#section-4.10
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Claim {
    /// The user subject. Also, the user_id.
    pub sub: Uuid,
    /// Used for session cache lookups
    pub jti: Uuid,
    /// Unix timestamp expiry
    pub exp: u64,
    /// A target audience server
    pub aud: String,
    /// Issuing subject
    pub iss: String,
    /// Issued at
    pub iat: u64,
}

const JWT_EXPIRY_SECS: u64 = Duration::days(7).num_seconds() as u64;

impl Claim {
    pub fn new(user_id: Uuid) -> Self {
        let now = jsonwebtoken::get_current_timestamp();
        Self {
            sub: user_id,
            jti: Uuid::new_v4(),
            iss: env!("CARGO_CRATE_NAME").to_string(),
            aud: env!("CARGO_CRATE_NAME").to_string(),
            iat: now,
            exp: now + JWT_EXPIRY_SECS,
        }
    }

    pub async fn get_user(&self, pool: &PgPool) -> Result<User, sqlx::Error> {
        let v = User::by_id(self.sub, pool).await?;

        v.ok_or_else(|| panic!("TODO:  Handle missing user from database"))
    }
}
