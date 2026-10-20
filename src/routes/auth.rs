use crate::auth::{Claim, Jwt, KVStore, Session};
use crate::models::db::users::User;
use crate::models::http::users::{
    CreateUserRequest, CreateUserResponse, SessionInvalidationResponse, UserDetailsResponse,
    UserLoginRequest, UserLoginResponse,
};
use crate::state::AxumApiState;
use argon2::{PasswordHash, PasswordHasher, PasswordVerifier};
use axum::Json;
use axum::extract::State;
use jsonwebtoken::Header;
use tracing::info;

pub const AUTH_TAG: &'static str = "auth";

use crate::error::{AppError, Result, StandardErrorResponses};
#[utoipa::path(get, path = "/auth/user/create", responses(
    (status = 200, description = "Get details about yourself", body = UserDetailsResponse),
    StandardErrorResponses
), tag = AUTH_TAG)]
/// Get details about the current user
pub async fn get_user(jwt: Jwt, State(state): State<AxumApiState>) -> Result<UserDetailsResponse> {
    info!("{:?}", jwt);
    let user = jwt.claim.get_user(&state.pool).await?;
    let res = UserDetailsResponse {
        username: user.username,
        email: user.email,
        created_at: user.created_at,
    };
    Ok(Json(res))
}

#[utoipa::path(post, path = "/auth/user/create", responses(
    (status = 200, description = "User created. Proceed to login", body = CreateUserResponse),
    StandardErrorResponses
), tag = AUTH_TAG)]
/// Create a new user
pub async fn create_user(
    State(state): State<AxumApiState>,
    Json(req): Json<CreateUserRequest>,
) -> Result<CreateUserResponse> {
    // Validate, then update
    let pass_hash = state
        .password_hasher
        .hash_password(req.password.as_ref())
        .map_err(AppError::from)?
        .to_string();

    let res = sqlx::query("INSERT INTO users(username, email, password_hash) VALUES ($1, $2, $3) ")
        .bind(req.username)
        .bind(req.email)
        .bind(pass_hash)
        .execute(&state.pool)
        .await
        .map_err(AppError::Database)?;

    info!("{:?}", res.rows_affected());

    let res = CreateUserResponse { success: true };
    Ok(Json(res))
}

#[utoipa::path(post, path = "/auth/user/session", responses(
    (status = 200, description = "User logged in", body = UserLoginResponse),
    StandardErrorResponses
), tag = AUTH_TAG)]
/// Log a user in. Expect to reuse the JWT for a duration.
pub async fn login_user(
    State(state): State<AxumApiState>,
    Json(req): Json<UserLoginRequest>,
) -> Result<UserLoginResponse> {
    let user = User::by_email(req.email, &state.pool)
        .await
        .map_err(AppError::Database)?;

    let user = match user {
        None => return Err(AppError::Authorization),
        Some(u) => u,
    };

    let hash = PasswordHash::new(&user.password_hash).map_err(AppError::from)?;
    let valid = state
        .password_hasher
        .verify_password(req.password.as_ref(), &hash);

    if valid.is_err() {
        return Err(AppError::Authorization);
    }

    let claim = Claim::new(user.id);
    let token = jsonwebtoken::encode(&Header::default(), &claim, &state.jwt_encode)
        .map_err(|e| AppError::Internal(e.into()))?;
    let session = Session {
        user_id: claim.sub,
        valid: true,
    };
    state
        .session_cache
        .insert(&claim.jti, session)
        .await
        .unwrap(); // only track sessions after creating the token for it

    let res = UserLoginResponse { token };

    Ok(Json(res))
}

#[utoipa::path(delete, path = "/auth/user/session", responses(
    (status = 200, description = "Successful session logout", body = SessionInvalidationResponse),
), tag = AUTH_TAG)]
/// Log the current user out.
pub async fn logout_user(
    session: Session,
    jwt: Jwt,
    state: State<AxumApiState>,
) -> Result<SessionInvalidationResponse> {
    info!("{:?}", session);
    state
        .session_cache
        .invalidate(&jwt.claim.jti)
        .await
        .unwrap();

    Ok(Json(SessionInvalidationResponse))
}

// TODO: invalidate all sessions
