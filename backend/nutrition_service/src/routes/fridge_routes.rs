use axum::{Router, routing::get, Json};
use crate::services::google_sheets_service::GoogleSheetsService;

pub async fn get_fridge_items_handler() -> Json<Vec<crate::models::FridgeItem>> {
    let service = GoogleSheetsService::new("YOUR_SPREADSHEET_ID").await.unwrap();
    let items = service.get_fridge_items().await.unwrap();
    Json(items)
}

pub fn fridge_routes() -> Router {
    Router::new()
        .route("/fridge-items", get(get_fridge_items_handler))
}
