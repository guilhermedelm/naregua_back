use axum::{
    routing::{get, post};
    Router
}
#[tokio::main]

pub fn rotas() -> Router{
    Router::new()
        .route("/", get(hello))
        .route("/register", post(register_user))

}