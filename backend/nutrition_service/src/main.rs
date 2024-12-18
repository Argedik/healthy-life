use axum::{Router};
use tokio;

mod models;
mod services;
mod routes;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .merge(routes::fridge_routes::fridge_routes());

    axum::Server::bind(&"0.0.0.0:8081".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
