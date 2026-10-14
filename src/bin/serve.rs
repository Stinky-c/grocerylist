use grocerylist::app;
use std::net::SocketAddr;
use tokio::signal;
use tracing::info;

#[tokio::main]
async fn main() {
    let (router, _) = app::make_app().await;
    let addr = std::env::var("APP_HOST")
        .unwrap_or("127.0.0.1:3000".into())
        .parse::<SocketAddr>()
        .unwrap();

    info!("listening on {addr}");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("TODO: panic message");
    info!("Shutting down")
}

// https://github.com/tokio-rs/axum/blob/e57b547eb95de74b73b34719dd5926254a085634/examples/graceful-shutdown/src/main.rs
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
