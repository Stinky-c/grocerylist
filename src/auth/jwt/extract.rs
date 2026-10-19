use crate::ApiErrorResponse;
use crate::auth::Claim;
use crate::error::ApiError;
use crate::state::AxumApiState;
use axum::extract::FromRequestParts;
use axum::http::StatusCode;
use axum::http::request::Parts;
use axum_extra::TypedHeader;
use jsonwebtoken::errors::ErrorKind;
use tracing::debug;

type BearerAuthorization =
    axum_extra::headers::Authorization<axum_extra::headers::authorization::Bearer>;

#[derive(Clone, Debug)]
pub struct Jwt {
    pub claim: Claim,
    pub header: jsonwebtoken::Header,
}

impl FromRequestParts<AxumApiState> for Jwt {
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AxumApiState,
    ) -> Result<Self, Self::Rejection> {
        let header = TypedHeader::<BearerAuthorization>::from_request_parts(parts, &())
            .await
            .map_err(|_| ApiError::BadRequest)?;

        // decode performs validation
        let token =
            jsonwebtoken::decode::<Claim>(header.token(), &state.jwt_decode, &state.jwt_validation)
                .map_err(map_jwt_error)?;

        debug!("Decoded: {:?}", token);

        Ok(Self {
            claim: token.claims,
            header: token.header,
        })
    }
}

fn map_jwt_error(e: jsonwebtoken::errors::Error) -> ApiError {
    match e.kind() {
        // Malformed requests
        ErrorKind::Json(_)
        | ErrorKind::Base64(_)
        | ErrorKind::InvalidToken
        | ErrorKind::Utf8(_) => ApiError::BadRequest,

        // Bad user
        ErrorKind::ExpiredSignature
        | ErrorKind::InvalidIssuer
        | ErrorKind::InvalidAudience
        | ErrorKind::InvalidSubject
        | ErrorKind::ImmatureSignature
        | ErrorKind::InvalidSignature => ApiError::Unauthorized,

        // Server errors
        ErrorKind::InvalidEcdsaKey
        | ErrorKind::InvalidEddsaKey
        | ErrorKind::InvalidRsaKey(_)
        | ErrorKind::RsaFailedSigning
        | ErrorKind::InvalidAlgorithmName
        | ErrorKind::UnsupportedAlgorithm
        | ErrorKind::InvalidKeyFormat
        | ErrorKind::MissingRequiredClaim(_)
        | ErrorKind::InvalidClaimFormat(_)
        | ErrorKind::InvalidAlgorithm
        | ErrorKind::MissingAlgorithm
        | ErrorKind::Provider(_)
        | ErrorKind::Signing(_)
        | _ => ApiError::InternalServerError,
    }
}
