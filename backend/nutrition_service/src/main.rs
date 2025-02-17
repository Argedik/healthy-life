use actix_web::{get, post, web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use actix_cors::Cors;

#[derive(Serialize, Deserialize)]
struct FridgeItem {
    id: u32,
    title: String,
    image_url: String,
}

#[get("/fridge_items")]
async fn get_fridge_items() -> impl Responder {
    // Şimdilik sabit bir liste döndürelim
    let items = vec![
        FridgeItem { id: 1, title: "Elma".to_string(), image_url: "http://...".to_string() },
        FridgeItem { id: 2, title: "Armut".to_string(), image_url: "http://...".to_string() },
    ];
    HttpResponse::Ok().json(items)
}

#[post("/fridge_items")]
async fn create_fridge_item(item: web::Json<FridgeItem>) -> impl Responder {
    // Burada veritabanına ekleyebilirsin. Biz sadece gelen item'i döndürüyoruz
    HttpResponse::Ok().json(item.0)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting nutrition service on port 8080...");
    HttpServer::new(|| {
        App::new()
            .wrap(Cors::default().allow_any_origin().allow_any_method().allow_any_header())
            .service(get_fridge_items)
            .service(create_fridge_item)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
