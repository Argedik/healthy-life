use actix_web::{get, post, web, HttpResponse, Responder};
use crate::models::FoodItem;

#[post("/add_food")]
async fn add_food(item: web::Json<FoodItem>) -> impl Responder {
  //Veritabanına ekleme işlemi
  HttpResponse::Ok().json("Food item added")
}

#[get("/get_food/{id}")]
async fn get_food(web::Path(id): web::Path<i32>) -> impl Responder {
  HttpResponse::Ok().json(FoodItem {
    id,
    name: "Elma".to_string(),
    calories: 52,
    proteins: 0.3,
    carbs: 14.0,
    fats: 0.2,
  })
}