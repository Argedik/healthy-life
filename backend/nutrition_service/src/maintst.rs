use axum::{Router, routing::{get, put, delete}, extract::Query, Json};
use std::net::SocketAddr;
use serde::Deserialize;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tokio::net::TcpListener;

mod google_sheets_client;
mod models;
mod services;
mod controller;

use services::google_sheets_service::{
    get_fridge_items_from_sheet, add_fridge_item, update_fridge_item, clear_fridge_item
};
use models::fridge_item::FridgeItem;

#[derive(Deserialize)]
struct SheetQuery {
    spreadsheet_id: String,
}

#[derive(Deserialize)]
struct UpdateQuery {
    spreadsheet_id: String,
    row: u32,
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    let listener = TcpListener::bind(addr).await.unwrap();

    let app = Router::new()
        .route("/fridge_items", get(get_items).post(create_item))
        .route("/fridge_items/update", put(update_item))
        .route("/fridge_items/delete", delete(delete_item))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http()) // İsteği izlemek için TraceLayer ekleyebilirsiniz
        );

    println!("Server çalışıyor: http://{}", addr);

    axum::serve(listener, app).await.unwrap();
}

// GET -> Listele
async fn get_items(Query(params): Query<SheetQuery>) -> Json<Vec<FridgeItem>> {
    let items = get_fridge_items_from_sheet(&params.spreadsheet_id).await.unwrap_or_default();
    Json(items)
}

// POST -> Ekle
async fn create_item(Query(params): Query<SheetQuery>, Json(item): Json<FridgeItem>) -> Json<bool> {
    let res = add_fridge_item(&params.spreadsheet_id, item).await;
    Json(res.is_ok())
}

// PUT -> Güncelle
async fn update_item(Query(params): Query<UpdateQuery>, Json(item): Json<FridgeItem>) -> Json<bool> {
    let res = update_fridge_item(&params.spreadsheet_id, params.row, item).await;
    Json(res.is_ok())
}

// DELETE -> Sil
async fn delete_item(Query(params): Query<UpdateQuery>) -> Json<bool> {
    let res = clear_fridge_item(&params.spreadsheet_id, params.row).await;
    Json(res.is_ok())
}
