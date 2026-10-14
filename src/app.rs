use crate::state::ApiState;
use axum::Router;
use sqlx::migrate::Migrator;
use std::sync::Arc;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use utoipa::openapi::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use crate::routes;

static MIGRATOR: Migrator = sqlx::migrate!();

#[tracing::instrument]
pub async fn make_app() -> (Router, OpenApi) {
    dotenvy::dotenv().expect("Failed to read .env file");
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db_url = std::env::var("DATABASE_URL").unwrap();

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .unwrap();

    MIGRATOR.run(&pool).await.unwrap();

    let state = ApiState { pool };

    // build our application with some routes
    let (router, api) = OpenApiRouter::new()
        .routes(utoipa_axum::routes!(
            routes::auth::list_users,
            routes::auth::create_user
        ))
        .with_state(Arc::new(state))
        .split_for_parts();

    let app: Router = Router::new().nest("/api/v1/", router);

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