use axum::{Json, extract::Query};
use serde::{Deserialize, Serialize};
use crate::models::fridge_item::FridgeItem;
use crate::services::google_sheets_service::{get_fridge_items_from_sheet, add_fridge_item, update_fridge_item, clear_fridge_item};

#[derive(Deserialize)]
pub struct SheetQuery {
    pub spreadsheet_id: String,
}

// GET (Read)
pub async fn get_fridge_items(Query(params): Query<SheetQuery>) -> Json<Vec<FridgeItem>> {
    let items = get_fridge_items_from_sheet(&params.spreadsheet_id).await.unwrap_or_default();
    Json(items)
}

// CREATE
pub async fn create_fridge_item(Query(params): Query<SheetQuery>, Json(item): Json<FridgeItem>) -> Json<bool> {
    let res = add_fridge_item(&params.spreadsheet_id, item).await;
    Json(res.is_ok())
}

// UPDATE
// Burada hangi satırı güncelleyeceğimizi biliyor olmamız lazım. 
// Örneğin satır parametresini da Query veya Path ile alabiliriz.
#[derive(Deserialize)]
pub struct UpdateQuery {
    pub spreadsheet_id: String,
    pub row: u32,
}

pub async fn update_fridge_item_handler(Query(params): Query<UpdateQuery>, Json(item): Json<FridgeItem>) -> Json<bool> {
    let res = update_fridge_item(&params.spreadsheet_id, params.row, item).await;
    Json(res.is_ok())
}

// DELETE
// Silmek yerine clear ediyoruz.
pub async fn delete_fridge_item_handler(Query(params): Query<UpdateQuery>) -> Json<bool> {
    let res = clear_fridge_item(&params.spreadsheet_id, params.row).await;
    Json(res.is_ok())
}
