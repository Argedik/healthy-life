// Path: URL’deki path parametresi almak için (mesela /fridge_items/:id).
// StatusCode: HTTP durum kodlarını döndürebilmek için.
// Json: Gelen/giden veriyi JSON olarak parse veya serileştirmek için.
use axum::{
    extract::{Path},
    http::StatusCode,
    Json,
};

//Handler’da parametre veya dönüş tipi olarak FridgeItem struct’ına erişiyoruz.
use crate::models::fridge_item::FridgeItem;

//“db_service” modülündeki CRUD fonksiyonları.
use crate::services::db_services::{
    add_fridge_item,
    get_all_fridge_items,
    update_fridge_item,
    delete_fridge_item,
};

/// GET -> Tüm kayıtları döndür
pub async fn get_fridge_items() -> Result<Json<Vec<FridgeItem>>, StatusCode> {
    match get_all_fridge_items().await {
        Ok(items) => Ok(Json(items)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// POST -> Yeni kayıt ekle
pub async fn create_fridge_item(
    Json(item): Json<FridgeItem>,
) -> Result<Json<bool>, StatusCode> {
    match add_fridge_item(item).await {
        Ok(_) => Ok(Json(true)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// PUT -> Güncelle (/fridge_items/:id)
pub async fn update_fridge_item_handler(
    Path(id): Path<i64>,
    Json(item): Json<FridgeItem>,
) -> Result<Json<bool>, StatusCode> {
    match update_fridge_item(id, item).await {
        Ok(_) => Ok(Json(true)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// DELETE -> Sil (/fridge_items/:id)
pub async fn delete_fridge_item_handler(
    Path(id): Path<i64>,
) -> Result<Json<bool>, StatusCode> {
    match delete_fridge_item(id).await {
        Ok(_) => Ok(Json(true)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
