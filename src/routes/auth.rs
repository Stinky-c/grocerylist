use crate::auth::{Claim, Jwt, KVStore, Session};
use crate::error::ApiError;
use crate::models::db::users::User;
use crate::models::http::users::{
    CreateUserRequest, CreateUserResponse, SessionInvalidationResponse, UserDetailsResponse,
    UserLoginRequest, UserLoginResponse,
};
use crate::state::AxumApiState;
use crate::{ApiErrorResponse, error};
use argon2::{PasswordHash, PasswordHasher, PasswordVerifier};
use axum::Json;
use axum::extract::State;
use jsonwebtoken::Header;
use tracing::info;

#[utoipa::path(get, path = "/auth/user/create", responses(
    (status = 200, description = "Get details about yourself", body = UserDetailsResponse),
    (status = 401, description = "Missing auth", body = ApiErrorResponse),
))]
pub async fn get_user(
    jwt: Jwt,
    State(state): State<AxumApiState>,
) -> error::Result<UserDetailsResponse> {
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
    ApiError
))]
#[axum::debug_handler]
pub async fn create_user(
    State(state): State<AxumApiState>,
    Json(req): Json<CreateUserRequest>,
) -> error::Result<CreateUserResponse> {
    // Validate, then update
    let pass_hash = state
        .password_hasher
        .hash_password(req.password.as_ref())
        .map_err(|_| ApiError::Argon2HashError)?
        .to_string();

    let res = sqlx::query("INSERT INTO users(username, email, password_hash) VALUES ($1, $2, $3) ")
        .bind(req.username)
        .bind(req.email)
        .bind(pass_hash)
        .execute(&state.pool)
        .await
        .map_err(|_| ApiError::SqlxError)?;

    info!("{:?}", res.rows_affected());

    let res = CreateUserResponse { success: true };
    Ok(Json(res))
}

#[utoipa::path(post, path = "/auth/user/session", responses(
    (status = 200, description = "User logged in", body = UserLoginResponse),
))]
pub async fn login_user(
    State(state): State<AxumApiState>,
    Json(req): Json<UserLoginRequest>,
) -> error::Result<UserLoginResponse> {
    let user = User::by_email(req.email, &state.pool)
        .await
        .map_err(|_| ApiError::InternalServerError)?;

    let user = match user {
        None => return Err(ApiError::Unauthorized),
        Some(u) => u,
    };

    let hash = PasswordHash::new(&user.password_hash).map_err(|_| ApiError::InternalServerError)?;
    let valid = state
        .password_hasher
        .verify_password(req.password.as_ref(), &hash);

    if valid.is_err() {
        return Err(ApiError::Unauthorized);
    }

    let claim = Claim::new(user.id);
    let token = jsonwebtoken::encode(&Header::default(), &claim, &state.jwt_encode)
        .map_err(|_| ApiError::InternalServerError)?;
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
))]
pub async fn logout_user(
    session: Session,
    jwt: Jwt,
    state: State<AxumApiState>,
) -> error::Result<SessionInvalidationResponse> {
    info!("{:?}", session);
    state
        .session_cache
        .invalidate(&jwt.claim.jti)
        .await
        .unwrap();

    Ok(Json(SessionInvalidationResponse))
}

// TODO: invalidate all sessions
