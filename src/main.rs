use axum::Router;
use tower_http::cors::CorsLayer;

// Declaramos que nosso projeto tem esses três módulos (pastas)
mod models;
mod handlers;
mod routes;


#[tokio::main]
async fn main() {
    let app = Router::new()
        .nest("/users", routes::users::rotas());
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("0.0.0.0.3000")
        .await
        .unwrap();    

    println!("Server running on http://0.0.0.0.3000");

    axum::serve(listener, app)
        .await
        .unwrap();
}