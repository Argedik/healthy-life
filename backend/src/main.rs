use axum::{
  routing::{get, post},
  Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use std::{net::SocketAddr, path::Path};

#[derive(Serialize, Deserialize, Debug)]
struct Food {
  id: i64,
  name: String,
  calories: i32,
}

#[tokio::main]
async fn main() {
  let db_path = "C:\\Users\\enes.gedik\\Desktop\\fe\\besin-uygulamasi\\database\\app_data.sqlite";

  // SQLite veritabanı bağlantısı
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
      .route("/foods", get(get_foods).post(create_food))
      .with_state(pool);

  let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
  println!("Backend başlatıldı: http://{}", addr);

  axum::Server::bind(&addr)
      .serve(app.into_make_service())
      .await
      .unwrap();
}

async fn get_foods(pool: axum::extract::State<SqlitePool>) -> Json<Vec<Food>> {
  let foods = sqlx::query_as!(
      Food,
      "SELECT id, name, calories FROM foods"
  )
  .fetch_all(&*pool)
  .await
  .unwrap();

  Json(foods)
}

async fn create_food(
  Json(payload): Json<Food>,
  pool: axum::extract::State<SqlitePool>,
) -> Json<Food> {
  let new_food = sqlx::query_as!(
      Food,
      "INSERT INTO foods (name, calories) VALUES (?, ?) RETURNING id, name, calories",
      payload.name,
      payload.calories
  )
  .fetch_one(&*pool)
  .await
  .unwrap();

  Json(new_food)
}
