use actix_web::{HttpServer, App};
mod handlers;
mod routes;
mod db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    db::init_db();

    HttpServer::new(|| {
        App::new()
            .configure(routes::config)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}