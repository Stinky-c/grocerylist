use tokio::fs;
use grocerylist::app::make_app;

#[tokio::main]
async fn main() {
    let (_, api) = make_app().await;

    let json = api.to_pretty_json().unwrap();
    fs::write("./openapi.json", json).await.unwrap();
}
