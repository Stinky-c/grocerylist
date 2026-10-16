use crate::ApiErrorResponse;
use crate::auth::{Claim, Jwt};
use crate::error::AuthError;
use crate::models::db::users::User;
use crate::models::http::users::{
    CreateUserRequest, CreateUserResponse, UserDetailsResponse, UserLoginRequest,
};
use crate::state::AxumApiState;
use argon2::PasswordHasher;
use axum::Json;
use axum::extract::State;
use axum::response::{IntoResponse, Result};
use tracing::{debug, info};

#[utoipa::path(get, path = "/auth/user", responses(
    (status = 200, description = "Get details about yourself", body = UserDetailsResponse),
    (status = 401, description = "Missing auth", body = ApiErrorResponse),
    (status = 418, description = "Bad request", body = ApiErrorResponse)
))]
#[axum::debug_handler]
pub async fn get_user(
    _: Jwt,
    State(state): State<AxumApiState>,
) -> Result<Json<UserDetailsResponse>> {
    Err(AuthError::Unauthorized.into())
}

#[utoipa::path(post, path = "/auth/user", responses(
    (status = 200, description = "User created. Proceed to login", body = CreateUserResponse),
    (status = 401, description = "Bad auth", body = ApiErrorResponse), // vague to prevent fuzzing
))]
#[axum::debug_handler]
pub async fn create_user(
    State(state): State<AxumApiState>,
    Json(req): Json<CreateUserRequest>,
) -> Result<Json<CreateUserResponse>> {
    let pass_hash = state
        .password_hasher
        .hash_password(req.password.as_ref())
        .map_err(|_| AuthError::InternalServerError)?
        .to_string();

    let res = sqlx::query("INSERT INTO users(username, email, password_hash) VALUES ($1, $2, $3) ")
        .bind(req.username)
        .bind(req.email)
        .bind(pass_hash)
        .execute(&state.pool)
        .await
        .map_err(|_| AuthError::InternalServerError)?;
    info!("{:?}", res.rows_affected());

    let res = CreateUserResponse { success: true };
    Ok(Json(res))
}

#[utoipa::path(post, path = "/auth/user/login")]
#[axum::debug_handler]
pub async fn login_user(
    State(state): State<AxumApiState>,
    Json(req): Json<UserLoginRequest>,
) -> Result<Json<UserDetailsResponse>> {
    let user: Option<User> = User::by_email(req.email, &state.pool)
        .await
        .map_err(|_| AuthError::InternalServerError)?;

    let user = match user {
        None => return Err(AuthError::InvalidCredentials.into()),
        Some(u) => u,
    };

    // Claim::new(user)

    Err(AuthError::Unauthorized.into())
}

#[utoipa::path(post, path = "/auth/user/logout")]
#[axum::debug_handler]
pub async fn logout_user() -> Result<Json<UserDetailsResponse>> {
    // Invalidate provided session here
    Err(AuthError::Unauthorized.into())
}

#[utoipa::path(delete, path = "/auth/user/logout")]
pub async fn invalidate_sessions() -> Result<Json<UserDetailsResponse>> {
    Err(AuthError::Unauthorized.into())
}
