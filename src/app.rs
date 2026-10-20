use crate::auth::MemorySessionCache;
use crate::config::AppConfig;
use crate::openapi::ApiDoc;
use crate::routes;
use crate::state::ApiState;
use axum::Router;
use axum::http::StatusCode;
use sqlx::migrate::Migrator;
use std::sync::Arc;
use std::time::Duration;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, fmt};
use utoipa::OpenApi as _;
use utoipa::openapi::OpenApi;
use utoipa_axum::router::OpenApiRouter;

static MIGRATOR: Migrator = sqlx::migrate!();

#[tracing::instrument]
pub async fn make_app() -> (Router, OpenApi) {
    dotenvy::dotenv().expect("Failed to read .env file");

    tracing_subscriber::registry()
        .with(fmt::layer().with_line_number(true))
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .init();

    let config = AppConfig::load();

    let pool = config.database().await;

    MIGRATOR.run(&pool).await.unwrap();

    let (jwt_enc, jwt_dec) = config.jwt_pair();
    let hasher = config.password_hasher();
    let validation = config.validation();

    let state = ApiState {
        pool,
        jwt_decode: Arc::new(jwt_dec),
        jwt_encode: Arc::new(jwt_enc),
        jwt_validation: Arc::new(validation),
        password_hasher: Arc::new(hasher),
        session_cache: MemorySessionCache::new(),
    };

    // build our application with some routes
    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .routes(utoipa_axum::routes!(
            routes::auth::get_user,
            routes::auth::create_user
        ))
        .routes(utoipa_axum::routes!(
            routes::auth::login_user,
            routes::auth::logout_user
        ))
        .with_state(Arc::new(state))
        .split_for_parts();

    let app: Router = Router::new().nest("/api/v1/", router).layer((
        TraceLayer::new_for_http(),
        TimeoutLayer::with_status_code(StatusCode::REQUEST_TIMEOUT, Duration::from_secs(10)),
    ));

    #[cfg(feature = "swagger")]
    let app = {
        #[cfg(not(debug_assertions))]
        compile_error!("Swagger cannot be enabled for release builds");

        use tracing::warn;
        use utoipa_swagger_ui::SwaggerUi;
        warn!("Swagger UI enabled!");
        app.merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api.clone()))
    };

    (app, api)
}
