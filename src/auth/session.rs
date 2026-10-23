use crate::auth::{Jwt, format_jti};
use crate::error::AppError;
use crate::state::ApiState;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use fred::interfaces::KeysInterface;
use serde::{Deserialize, Serialize};
use serde_json::{Error, Value};
use uuid::Uuid;

/// A user session. One user can have many session and typically correlate to a single JWT
#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    pub(crate) user_id: Uuid,
    pub(crate) valid: bool,
}

impl Session {
    pub fn to_value(self) -> Result<Value, Error> {
        serde_json::to_value(&self)
    }

    pub fn from_value(obj: Value) -> Result<Session, Error> {
        serde_json::from_value(obj)
    }

    pub fn to_string(self) -> Result<String, Error> {
        serde_json::to_string(&self)
    }

    pub fn from_string(s: String) -> Result<Session, Error> {
        serde_json::from_str(&s)
    }
}

impl FromRequestParts<ApiState> for Session {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &ApiState,
    ) -> Result<Self, Self::Rejection> {
        let jwt = Jwt::from_request_parts(parts, state).await?;
        let jti = format_jti(&jwt.claim.jti);
        let value: String = state.kv.get(jti).await?;

        Ok(Self::from_string(value)?)
    }
}
