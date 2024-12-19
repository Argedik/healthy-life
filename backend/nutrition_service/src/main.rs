use axum::{
    routing::{get, post, put, delete},
    Router,
  };
  use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
  use std::{net::SocketAddr, path::Path};
  
  mod google_sheets_client;
  mod models;
  mod services;
  mod controller;
  
  #[tokio::main]
  async fn main() {
    let db_path = "C:\\Users\\enes.gedik\\Desktop\\fe\\besin-uygulamasi\\database\\app_data.sqlite";
  
    if !Path::new(db_path).exists() {
        eprintln!("Veritabanı dosyası oluşturuluyor...");
    }
  
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&format!("sqlite://{}", db_path))
        .await
        .expect("Veritabanına bağlanılamadı");
  
    // Tabloyu oluştur
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS foods (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            calories INTEGER NOT NULL
        );",
    )
    .execute(&pool)
    .await
    .unwrap();
  
    let app = Router::new()
        // Foods CRUD
        .route("/foods", get(foods_controller::get_foods).post(foods_controller::create_food))
        .route("/foods/:id", get(foods_controller::get_food)
                            .put(foods_controller::update_food)
                            .delete(foods_controller::delete_food))
        // Fridge Items (Google Sheets)
        .route("/fridge_items", get(fridge_items_controller::get_fridge_items)
                               .post(fridge_items_controller::create_fridge_item))
        .route("/fridge_items/update", put(fridge_items_controller::update_fridge_item_handler))
        .route("/fridge_items/delete", delete(fridge_items_controller::delete_fridge_item_handler))
        .with_state(pool);
  
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Backend başlatıldı: http://{}", addr);
  
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
  }
  