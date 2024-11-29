use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct FoodItem {
  pub id: i32,
  pub name: String,
  pub calories: i32,
  pub proteins: f32,
  pub carbs: f32,
  pub fats: f32,
}