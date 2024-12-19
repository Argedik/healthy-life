use axum::{Json, extract::State};
use serde::{Deserialize, Serialize};
use sqlx::{SqlitePool};

#[derive(Serialize, Deserialize, Debug)]
pub struct Food {
    pub id: i64,
    pub name: String,
    pub calories: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateFood {
    pub name: String,
    pub calories: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateFood {
    pub id: i64,
    pub name: String,
    pub calories: i32,
}

// CREATE
pub async fn create_food(State(pool): State<SqlitePool>, Json(payload): Json<CreateFood>) -> Json<Food> {
    let new_food = sqlx::query_as!(
        Food,
        "INSERT INTO foods (name, calories) VALUES (?, ?) RETURNING id, name, calories",
        payload.name,
        payload.calories
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    Json(new_food)
}

// READ (Tüm veriler)
pub async fn get_foods(State(pool): State<SqlitePool>) -> Json<Vec<Food>> {
    let foods = sqlx::query_as!(
        Food,
        "SELECT id, name, calories FROM foods"
    )
    .fetch_all(&pool)
    .await
    .unwrap();

    Json(foods)
}

// READ (Tek bir kayıt)
pub async fn get_food(State(pool): State<SqlitePool>, axum::extract::Path(id): axum::extract::Path<i64>) -> Json<Option<Food>> {
    let food = sqlx::query_as!(
        Food,
        "SELECT id, name, calories FROM foods WHERE id = ?",
        id
    )
    .fetch_optional(&pool)
    .await
    .unwrap();

    Json(food)
}

// UPDATE
pub async fn update_food(State(pool): State<SqlitePool>, Json(payload): Json<UpdateFood>) -> Json<Option<Food>> {
    let food = sqlx::query_as!(
        Food,
        "UPDATE foods SET name = ?, calories = ? WHERE id = ? RETURNING id, name, calories",
        payload.name,
        payload.calories,
        payload.id
    )
    .fetch_optional(&pool)
    .await
    .unwrap();

    Json(food)
}

// DELETE
pub async fn delete_food(State(pool): State<SqlitePool>, axum::extract::Path(id): axum::extract::Path<i64>) -> Json<bool> {
    let result = sqlx::query!(
        "DELETE FROM foods WHERE id = ?",
        id
    )
    .execute(&pool)
    .await
    .unwrap();

    Json(result.rows_affected() > 0)
}
