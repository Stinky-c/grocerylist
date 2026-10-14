use grocerylist::app;
use std::net::SocketAddr;
use tracing::info;

#[tokio::main]
async fn main() {
    let (app, _) = app::make_app().await;
    let addr = std::env::var("APP_HOST")
        .unwrap_or("127.0.0.1:3000".into())
        .parse::<SocketAddr>()
        .unwrap();

    info!("listening on {addr}");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app)
        .await
        .expect("TODO: panic message");
}
