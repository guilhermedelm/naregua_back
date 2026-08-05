

use axum::{
    routing::{get, post},
    Router,
    Json,
};

use serde::{Deserialize, Serialize};
use tower_http::cors::CorsLayer;

#[derive(Serialize, Deserialize)]
struct Message{
    message:String,
}

async fn hello() -> Json<Message> {
    Json(Message {
        message: "Hello, World!".into(),
    })
}
#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello))
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("0.0.0:3000")
    .await
    .unwrap();

    println!("Servidor em http://localhost:3000");

    axum::serve(listener, app)
        .await
        .unwrap();
}