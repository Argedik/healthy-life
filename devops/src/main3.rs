use actix_web::{web, App, HttpResponse, HttpServer, Responder};

// Dinamik olarak isim alan bir handler oluşturuyoruz
async fn greet(name: web::Path<String>) -> impl Responder {
    let response = format!("Merhaba, {}!", name);
    HttpResponse::Ok().body(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(|| async { HttpResponse::Ok().body("Ana Sayfa") }))
            .route("/hello/{name}", web::get().to(greet)) // "/hello/{name}" route'u ekleniyor
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
