mod handlers;
mod models;
mod routes;
mod db;

use handlers::AppState;
use routes::app_router;
use redis::Client as RedisClient;
use tracing_subscriber;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let pool = db::init_db().await;

    // Redis bağlantısı
    let redis_client = RedisClient::open("redis://127.0.0.1:6379").expect("Redis'e bağlanılamadı");

    let state = AppState {
        pool,
        redis: redis_client,
    };

    let app = app_router(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8081));
    println!("nutrition_service run at: {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
