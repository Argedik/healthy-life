use actix_web::{web, App, HttpServer, Responder, HttpResponse, get};
use std::env;
use dotenv::dotenv;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Rust Web Sunucusu Çalışıyor!")
}

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({ "status": "OK" }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT env değişkeni sayı olmalı");

    println!("🔵 Sunucu {} portunda çalışıyor...", port);

    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(health)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
