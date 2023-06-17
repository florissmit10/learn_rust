use std::net::SocketAddr;
use axum::{response::IntoResponse, routing::get, Router};


async fn hello_world() -> impl IntoResponse {
    "Hello world"
}


#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(hello_world));

    let addr = SocketAddr::new([0, 0, 0, 0].into(), 3000);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}