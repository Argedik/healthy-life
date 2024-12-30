use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FridgeItem {
    pub id: Option<i64>,      
    pub image_url: String,
    pub title: String,
}
