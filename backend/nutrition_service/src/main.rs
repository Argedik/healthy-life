use actix_web::{get, App, HttpServer, Responder};
use env_logger;

#[get("/health")]
async fn health_check() -> impl Responder {
    "Nutrition service is up!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Logger başlatma
    env_logger::init();
    println!("Starting nutrition service on port 8080...");

    HttpServer::new(|| {
        App::new()
            // İleride ekleyeceğimiz route’lar ve servisler burada toplanır
            .service(health_check)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
