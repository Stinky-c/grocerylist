use axum::extract::{FromRef, State};
use std::sync::Arc;

pub type AxumAppState = State<Arc<AppState>>;
pub type AxumApiState = State<Arc<ApiState>>;

#[derive(Clone)]
pub struct AppState {
    pub api_state: ApiState,
}

// Make sure everything inside is cheap to clone
#[derive(Debug, Clone)]
pub struct ApiState {
    pub pool: sqlx::postgres::PgPool,
}

impl FromRef<AppState> for ApiState {
    fn from_ref(input: &AppState) -> Self {
        input.api_state.clone()
    }
}
