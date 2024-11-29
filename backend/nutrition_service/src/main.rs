use actix_web::{web, App, HttpServer};
mod handlers;
mod models;
mod routes;
mod db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  db::init_db().await;

  HttpServer::new (|| {
    App::new().configure(routes::config)
  })
  .bind("0.0.0.0:8081")?
  .run()
  .await
}