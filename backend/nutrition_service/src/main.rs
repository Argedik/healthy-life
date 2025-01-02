// Router: Axum’da HTTP route define.
use axum::{
    Router,
    routing::{get, post, put, delete},
};

// ServiceBuilder: Axum’un (daha doğrusu Tower’ın) middleware veya katmanlar eklemesini kolaylaştıran builder.
use tower::ServiceBuilder;

// SocketAddr: IP adresi ve port bilgisini temsil ediyor.
use std::net::SocketAddr;

// TraceLayer: Gelen/giden HTTP isteklerini loglamak veya izlemek için bir middleware (opsiyonel).
use tower_http::trace::TraceLayer;

use dotenvy::dotenv;

mod models;
mod services;
mod controller;

//“db_services” modülündeki CRUD fonksiyonları.
use controller::fridge_items_controller::{
    get_fridge_items,
    create_fridge_item,
    update_fridge_item_handler,
    delete_fridge_item_handler,
};

// async
#[tokio::main]
async fn main() {
   // .env yükle
  let _ = dotenv();
  // db_service.rs veritabanını başlat
  services::db_services::init_db()
      .await
      .expect("Veritabanı başlatılamadı!");

  // Routing.
  let app = Router::new()
      // GET ve POST -> /fridge_items
      .route("/fridge_items", get(get_fridge_items).post(create_fridge_item))
      // PUT ve DELETE -> /fridge_items/:id
      .route("/fridge_items/:id", put(update_fridge_item_handler).delete(delete_fridge_item_handler))
      .layer(
          ServiceBuilder::new()
              .layer(TraceLayer::new_for_http())
      );

    // Sunucu adresi (host + port)
    let router  = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(router).await.unwrap();
    println!("Sunucu çalışıyor: http://{}", router);
    
    // Sunucuyu başlat
    axum::serve(listener, app).await.unwrap();
}
