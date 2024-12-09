use axum::{extract::{Path, State}, Json};
use sqlx::SqlitePool;
use redis::Client as RedisClient;
use serde_json::json;
use crate::models::FoodItem;
use anyhow::Result;

#[derive(Clone)]
pub struct AppState {
    pub pool: SqlitePool,
    pub redis: RedisClient,
}

// POST /add_food
pub async fn add_food(
    State(state): State<AppState>,
    Json(item): Json<FoodItem>
) -> Json<serde_json::Value> {
    // Veritabanına ekleme işlemi
    let mut conn = state.pool.acquire().await.unwrap();
    let _ = sqlx::query!(
        "INSERT INTO foods (name, calories) VALUES (?, ?)",
        item.name,
        item.calories
    )
    .execute(&mut conn)
    .await;

    // İstenirse redis cache vb. eklenebilir
    Json(json!({"status": "Food item added"}))
}

// GET /get_food/:id
pub async fn get_food(
    Path(id): Path<i32>,
    State(state): State<AppState>,
) -> Json<FoodItem> {
    let mut conn = state.pool.acquire().await.unwrap();
    // Basit bir örnek, gerçek veri tabanından çekiyoruz.
    // Burada proteins, carbs, fats alanları tabloya eklenmemiş olabilir.
    // Örnek olarak sabit değer veya tabloya göre uyarlayın.
    let record = sqlx::query!(
        "SELECT id, name, calories FROM foods WHERE id = ?",
        id
    )
    .fetch_one(&mut conn)
    .await
    .expect("Food not found");

    Json(FoodItem {
        id: record.id,
        name: record.name,
        calories: record.calories,
        proteins: 0.3,
        carbs: 14.0,
        fats: 0.2,
    })
}
