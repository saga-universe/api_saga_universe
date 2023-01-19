use std::net::SocketAddr;
use axum::{response::Html, routing::get, Router};

pub mod database;
pub mod schema;

#[tokio::main]
async fn main() {

    let app = Router::new().route("/", get(handler));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Welcome to the saga universe API!</h1>")
}
