use crate::auth::{Jwt, KVStore};
use crate::error::ApiError;
use crate::state::AxumApiState;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Session {
    /// Matching to the database
    pub(crate) user_id: Uuid,
    /// Tracking to a valid jwt via jti. See [crate::auth::jwt::Claim]
    pub(crate) valid: bool,
}

impl FromRequestParts<AxumApiState> for Session {
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AxumApiState,
    ) -> Result<Self, Self::Rejection> {
        let claim = Jwt::from_request_parts(parts, state).await?.claim;
        let jti = claim.jti;

        state
            .session_cache
            .get(&jti)
            .await
            .map_err(|_| ApiError::InternalServerError)?
            .ok_or_else(|| ApiError::Unauthorized)
    }
}
